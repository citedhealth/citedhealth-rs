use std::time::Duration;

use crate::error::CitedHealthError;
use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://citedhealth.com";
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Builder for configuring a [`CitedHealth`] client.
///
/// # Example
///
/// ```no_run
/// # async fn example() -> Result<(), citedhealth::CitedHealthError> {
/// let client = citedhealth::CitedHealth::builder()
///     .base_url("https://staging.citedhealth.com")
///     .timeout(std::time::Duration::from_secs(60))
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct CitedHealthBuilder {
    base_url: String,
    timeout: Duration,
}

impl CitedHealthBuilder {
    fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    /// Set the API base URL.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.trim_end_matches('/').to_string();
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<CitedHealth, CitedHealthError> {
        let http = reqwest::Client::builder().timeout(self.timeout).build()?;
        Ok(CitedHealth {
            base_url: self.base_url,
            http,
        })
    }
}

/// Async client for the Cited Health REST API.
///
/// Provides access to evidence-based supplement research data including
/// ingredients, conditions, evidence links with grades (A-F), and
/// PubMed-indexed papers.
///
/// # Example
///
/// ```no_run
/// # async fn example() -> Result<(), citedhealth::CitedHealthError> {
/// let client = citedhealth::CitedHealth::new();
/// let ingredients = client.list_ingredients(Some("biotin"), None).await?;
/// println!("Found {} ingredients", ingredients.count);
/// # Ok(())
/// # }
/// ```
pub struct CitedHealth {
    base_url: String,
    http: reqwest::Client,
}

impl CitedHealth {
    /// Create a new client with default settings.
    ///
    /// Uses `https://citedhealth.com` as the base URL and a 30-second timeout.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
                .build()
                .expect("failed to build default HTTP client"),
        }
    }

    /// Create a builder for custom configuration.
    pub fn builder() -> CitedHealthBuilder {
        CitedHealthBuilder::new()
    }

    /// Send a GET request and deserialize the JSON response.
    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, CitedHealthError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        let status = resp.status();

        if status.is_success() {
            return Ok(resp.json().await?);
        }

        match status.as_u16() {
            404 => Err(CitedHealthError::NotFound {
                resource: path.to_string(),
            }),
            429 => {
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(60);
                Err(CitedHealthError::RateLimit { retry_after })
            }
            _ => {
                let message = resp.text().await.unwrap_or_default();
                Err(CitedHealthError::Api {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }

    // -- Ingredient endpoints -------------------------------------------------

    /// List ingredients with optional search and category filter.
    ///
    /// # Arguments
    ///
    /// * `q` - Search by name (case-insensitive).
    /// * `category` - Filter by category (e.g. "vitamins", "herbs").
    pub async fn list_ingredients(
        &self,
        q: Option<&str>,
        category: Option<&str>,
    ) -> Result<PaginatedResponse<Ingredient>, CitedHealthError> {
        let mut params = Vec::new();
        if let Some(q) = q {
            params.push(format!("q={}", urlencoding(q)));
        }
        if let Some(cat) = category {
            params.push(format!("category={}", urlencoding(cat)));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/api/ingredients/{}", query)).await
    }

    /// Get a single ingredient by slug.
    pub async fn get_ingredient(&self, slug: &str) -> Result<Ingredient, CitedHealthError> {
        self.get(&format!("/api/ingredients/{}/", slug)).await
    }

    // -- Evidence endpoints ---------------------------------------------------

    /// List evidence links with optional ingredient and condition filters.
    ///
    /// Returns the first matching evidence link when both filters are provided.
    ///
    /// # Arguments
    ///
    /// * `ingredient` - Filter by ingredient slug (e.g. "biotin").
    /// * `condition` - Filter by condition slug (e.g. "hair-loss").
    pub async fn list_evidence(
        &self,
        ingredient: Option<&str>,
        condition: Option<&str>,
    ) -> Result<PaginatedResponse<EvidenceLink>, CitedHealthError> {
        let mut params = Vec::new();
        if let Some(ing) = ingredient {
            params.push(format!("ingredient={}", urlencoding(ing)));
        }
        if let Some(cond) = condition {
            params.push(format!("condition={}", urlencoding(cond)));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/api/evidence/{}", query)).await
    }

    /// Get a single evidence link by numeric ID.
    pub async fn get_evidence(&self, id: u64) -> Result<EvidenceLink, CitedHealthError> {
        self.get(&format!("/api/evidence/{}/", id)).await
    }

    // -- Paper endpoints ------------------------------------------------------

    /// List PubMed-indexed papers with optional search and year filter.
    ///
    /// # Arguments
    ///
    /// * `q` - Search in title (case-insensitive).
    /// * `year` - Filter by publication year.
    pub async fn list_papers(
        &self,
        q: Option<&str>,
        year: Option<u32>,
    ) -> Result<PaginatedResponse<Paper>, CitedHealthError> {
        let mut params = Vec::new();
        if let Some(q) = q {
            params.push(format!("q={}", urlencoding(q)));
        }
        if let Some(y) = year {
            params.push(format!("year={}", y));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/api/papers/{}", query)).await
    }

    /// Get a single paper by PubMed ID (PMID).
    pub async fn get_paper(&self, pmid: &str) -> Result<Paper, CitedHealthError> {
        self.get(&format!("/api/papers/{}/", pmid)).await
    }
}

impl Default for CitedHealth {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
