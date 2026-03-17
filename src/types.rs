use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Paginated API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Total number of results across all pages.
    pub count: u64,
    /// URL of the next page, if any.
    pub next: Option<String>,
    /// URL of the previous page, if any.
    pub previous: Option<String>,
    /// Results for the current page.
    pub results: Vec<T>,
}

/// A supplement ingredient with dosage and form information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    /// Unique numeric identifier.
    pub id: u64,
    /// Display name.
    pub name: String,
    /// URL-friendly slug.
    pub slug: String,
    /// Category (e.g. "vitamins", "herbs", "amino-acids").
    #[serde(default)]
    pub category: String,
    /// Mechanism of action description.
    #[serde(default)]
    pub mechanism: String,
    /// Recommended dosage by context (e.g. "general" -> "500mg daily").
    #[serde(default)]
    pub recommended_dosage: HashMap<String, String>,
    /// Available supplement forms (e.g. "capsule", "powder", "liquid").
    #[serde(default)]
    pub forms: Vec<String>,
    /// Whether this ingredient is featured on the homepage.
    #[serde(default)]
    pub is_featured: bool,
}

/// A health condition linked to evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// URL-friendly slug.
    pub slug: String,
    /// Display name.
    pub name: String,
}

/// A PubMed-indexed research paper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    /// Unique numeric identifier.
    pub id: u64,
    /// PubMed identifier.
    pub pmid: String,
    /// Paper title.
    pub title: String,
    /// Journal name.
    #[serde(default)]
    pub journal: String,
    /// Year of publication.
    #[serde(default)]
    pub publication_year: Option<u32>,
    /// Type of study (e.g. "RCT", "meta-analysis", "cohort").
    #[serde(default)]
    pub study_type: String,
    /// Number of citations.
    #[serde(default)]
    pub citation_count: u64,
    /// Whether the paper is open access.
    #[serde(default)]
    pub is_open_access: bool,
    /// Direct link to the PubMed page.
    #[serde(default)]
    pub pubmed_link: String,
}

/// Minimal ingredient reference nested inside evidence links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedIngredient {
    /// URL-friendly slug.
    pub slug: String,
    /// Display name.
    pub name: String,
}

/// An evidence link between an ingredient and a condition.
///
/// Represents the research relationship with a grade (A-F),
/// study count, participant count, and effect direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceLink {
    /// Unique numeric identifier.
    pub id: u64,
    /// The ingredient being studied.
    pub ingredient: NestedIngredient,
    /// The condition being studied.
    pub condition: Condition,
    /// Evidence grade code (e.g. "A", "B", "C", "D", "F").
    #[serde(default)]
    pub grade: String,
    /// Human-readable grade label (e.g. "Strong Evidence").
    #[serde(default)]
    pub grade_label: String,
    /// Summary of the evidence.
    #[serde(default)]
    pub summary: String,
    /// Direction of the effect (e.g. "positive", "negative", "mixed").
    #[serde(default)]
    pub direction: String,
    /// Total number of studies reviewed.
    #[serde(default)]
    pub total_studies: u64,
    /// Total number of participants across studies.
    #[serde(default)]
    pub total_participants: u64,
}
