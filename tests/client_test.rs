use citedhealth::{CitedHealth, CitedHealthError};

fn mock_ingredient_json() -> String {
    r#"{
        "id": 1,
        "name": "Biotin",
        "slug": "biotin",
        "category": "vitamins",
        "mechanism": "Coenzyme for carboxylase enzymes involved in fatty acid synthesis",
        "recommended_dosage": {"general": "2500 mcg daily"},
        "forms": ["capsule", "tablet", "liquid"],
        "is_featured": true
    }"#
    .to_string()
}

fn mock_paginated_ingredients_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_ingredient_json()
    )
}

fn mock_evidence_json() -> String {
    r#"{
        "id": 42,
        "ingredient": {"slug": "biotin", "name": "Biotin"},
        "condition": {"slug": "hair-loss", "name": "Hair Loss"},
        "grade": "B",
        "grade_label": "Good Evidence",
        "summary": "Moderate evidence supports biotin supplementation for hair loss.",
        "direction": "positive",
        "total_studies": 12,
        "total_participants": 1580
    }"#
    .to_string()
}

fn mock_paginated_evidence_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_evidence_json()
    )
}

fn mock_paper_json() -> String {
    r#"{
        "id": 100,
        "pmid": "34567890",
        "title": "Effects of biotin supplementation on hair growth: a systematic review",
        "journal": "Journal of Dermatological Treatment",
        "publication_year": 2023,
        "study_type": "meta-analysis",
        "citation_count": 45,
        "is_open_access": true,
        "pubmed_link": "https://pubmed.ncbi.nlm.nih.gov/34567890/"
    }"#
    .to_string()
}

fn mock_paginated_papers_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_paper_json()
    )
}

fn client_for(server: &mockito::ServerGuard) -> CitedHealth {
    CitedHealth::builder()
        .base_url(&server.url())
        .build()
        .expect("failed to build test client")
}

// -- list_ingredients ---------------------------------------------------------

#[tokio::test]
async fn test_list_ingredients() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/?q=biotin")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_ingredients_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_ingredients(Some("biotin"), None).await.unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].name, "Biotin");
    assert_eq!(result.results[0].slug, "biotin");
    assert_eq!(result.results[0].category, "vitamins");
    assert!(result.results[0].is_featured);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_ingredients_with_category() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/?q=biotin&category=vitamins")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_ingredients_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client
        .list_ingredients(Some("biotin"), Some("vitamins"))
        .await
        .unwrap();

    assert_eq!(result.count, 1);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_ingredients_no_params() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_ingredients_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_ingredients(None, None).await.unwrap();

    assert_eq!(result.count, 1);
    mock.assert_async().await;
}

// -- get_ingredient -----------------------------------------------------------

#[tokio::test]
async fn test_get_ingredient() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/biotin/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_ingredient_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let ingredient = client.get_ingredient("biotin").await.unwrap();

    assert_eq!(ingredient.id, 1);
    assert_eq!(ingredient.name, "Biotin");
    assert_eq!(ingredient.slug, "biotin");
    assert_eq!(ingredient.forms, vec!["capsule", "tablet", "liquid"]);
    assert_eq!(
        ingredient.recommended_dosage.get("general").unwrap(),
        "2500 mcg daily"
    );
    mock.assert_async().await;
}

// -- list_evidence ------------------------------------------------------------

#[tokio::test]
async fn test_list_evidence() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            "/api/evidence/?ingredient=biotin&condition=hair-loss",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_evidence_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client
        .list_evidence(Some("biotin"), Some("hair-loss"))
        .await
        .unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results[0].grade, "B");
    assert_eq!(result.results[0].ingredient.slug, "biotin");
    assert_eq!(result.results[0].condition.slug, "hair-loss");
    assert_eq!(result.results[0].total_studies, 12);
    assert_eq!(result.results[0].total_participants, 1580);
    mock.assert_async().await;
}

// -- get_evidence -------------------------------------------------------------

#[tokio::test]
async fn test_get_evidence() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/evidence/42/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_evidence_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let evidence = client.get_evidence(42).await.unwrap();

    assert_eq!(evidence.id, 42);
    assert_eq!(evidence.grade, "B");
    assert_eq!(evidence.grade_label, "Good Evidence");
    assert_eq!(evidence.direction, "positive");
    mock.assert_async().await;
}

// -- list_papers --------------------------------------------------------------

#[tokio::test]
async fn test_list_papers() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/papers/?q=melatonin")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_papers_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_papers(Some("melatonin"), None).await.unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results[0].pmid, "34567890");
    assert_eq!(result.results[0].study_type, "meta-analysis");
    assert!(result.results[0].is_open_access);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_papers_with_year() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/papers/?q=biotin&year=2023")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_papers_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client
        .list_papers(Some("biotin"), Some(2023))
        .await
        .unwrap();

    assert_eq!(result.count, 1);
    mock.assert_async().await;
}

// -- get_paper ----------------------------------------------------------------

#[tokio::test]
async fn test_get_paper() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/papers/34567890/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paper_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let paper = client.get_paper("34567890").await.unwrap();

    assert_eq!(paper.id, 100);
    assert_eq!(paper.pmid, "34567890");
    assert_eq!(paper.publication_year, Some(2023));
    assert_eq!(paper.citation_count, 45);
    assert_eq!(
        paper.pubmed_link,
        "https://pubmed.ncbi.nlm.nih.gov/34567890/"
    );
    mock.assert_async().await;
}

// -- Error handling -----------------------------------------------------------

#[tokio::test]
async fn test_not_found() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/nonexistent/")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"detail": "Not found."}"#)
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.get_ingredient("nonexistent").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, CitedHealthError::NotFound { ref resource } if resource.contains("nonexistent"))
    );
    assert!(err.to_string().contains("not found"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_rate_limit() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/")
        .with_status(429)
        .with_header("retry-after", "120")
        .with_body(r#"{"detail": "Request was throttled."}"#)
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_ingredients(None, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CitedHealthError::RateLimit { retry_after } if retry_after == 120));
    assert!(err.to_string().contains("retry after 120s"));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_rate_limit_default_retry() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/")
        .with_status(429)
        .with_body(r#"{"detail": "Request was throttled."}"#)
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_ingredients(None, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, CitedHealthError::RateLimit { retry_after } if retry_after == 60));
    mock.assert_async().await;
}

#[tokio::test]
async fn test_server_error() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_ingredients(None, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, CitedHealthError::Api { status, ref message } if status == 500 && message.contains("Internal Server Error"))
    );
    mock.assert_async().await;
}

// -- Builder ------------------------------------------------------------------

#[tokio::test]
async fn test_builder_custom_base_url() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/ingredients/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_ingredients_json())
        .create_async()
        .await;

    let client = CitedHealth::builder()
        .base_url(&server.url())
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("failed to build client");

    let result = client.list_ingredients(None, None).await.unwrap();
    assert_eq!(result.count, 1);
    mock.assert_async().await;
}

#[test]
fn test_default_client() {
    let client = CitedHealth::default();
    // Just verify it can be constructed without panicking.
    drop(client);
}

// -- Condition endpoints ------------------------------------------------------

fn mock_condition_json() -> String {
    r#"{
        "slug": "hair-loss",
        "name": "Hair Loss",
        "description": "A condition involving partial or complete loss of hair.",
        "meta_description": "Evidence-based research on hair loss treatments and supplements.",
        "prevalence": "Affects approximately 50 million men and 30 million women in the US.",
        "symptoms": ["thinning hair", "receding hairline", "bald patches"],
        "risk_factors": ["genetics", "hormonal changes", "stress"],
        "is_featured": true
    }"#
    .to_string()
}

fn mock_paginated_conditions_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_condition_json()
    )
}

#[tokio::test]
async fn test_list_conditions() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/conditions/?q=hair")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_conditions_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_conditions(Some("hair")).await.unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results[0].slug, "hair-loss");
    assert_eq!(result.results[0].name, "Hair Loss");
    assert!(result.results[0].is_featured);
    assert_eq!(result.results[0].symptoms.len(), 3);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_conditions_no_params() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/conditions/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_conditions_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_conditions(None).await.unwrap();

    assert_eq!(result.count, 1);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_condition() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/conditions/hair-loss/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_condition_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let condition = client.get_condition("hair-loss").await.unwrap();

    assert_eq!(condition.slug, "hair-loss");
    assert_eq!(condition.name, "Hair Loss");
    assert!(!condition.description.is_empty());
    assert_eq!(condition.risk_factors, vec!["genetics", "hormonal changes", "stress"]);
    mock.assert_async().await;
}

// -- Glossary endpoints -------------------------------------------------------

fn mock_glossary_term_json() -> String {
    r#"{
        "slug": "rct",
        "term": "Randomized Controlled Trial",
        "short_definition": "A study where participants are randomly assigned to treatment or control groups.",
        "definition": "A randomized controlled trial (RCT) is the gold standard of clinical research.",
        "abbreviation": "RCT",
        "category": "research-methods"
    }"#
    .to_string()
}

fn mock_paginated_glossary_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_glossary_term_json()
    )
}

#[tokio::test]
async fn test_list_glossary() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/glossary/?q=randomized")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_glossary_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_glossary(Some("randomized")).await.unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results[0].slug, "rct");
    assert_eq!(result.results[0].abbreviation, "RCT");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_glossary_term() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/glossary/rct/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_glossary_term_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let term = client.get_glossary_term("rct").await.unwrap();

    assert_eq!(term.slug, "rct");
    assert_eq!(term.term, "Randomized Controlled Trial");
    assert_eq!(term.abbreviation, "RCT");
    assert_eq!(term.category, "research-methods");
    mock.assert_async().await;
}

// -- Guide endpoints ----------------------------------------------------------

fn mock_guide_json() -> String {
    r#"{
        "slug": "biotin-for-hair",
        "title": "Biotin for Hair Growth: What the Research Says",
        "content": "Biotin is a B-vitamin commonly used for hair health.",
        "category": "supplement-guides",
        "meta_description": "A comprehensive guide to biotin supplementation for hair health."
    }"#
    .to_string()
}

fn mock_paginated_guides_json() -> String {
    format!(
        r#"{{
            "count": 1,
            "next": null,
            "previous": null,
            "results": [{}]
        }}"#,
        mock_guide_json()
    )
}

#[tokio::test]
async fn test_list_guides() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/guides/?q=biotin")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_paginated_guides_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let result = client.list_guides(Some("biotin")).await.unwrap();

    assert_eq!(result.count, 1);
    assert_eq!(result.results[0].slug, "biotin-for-hair");
    assert_eq!(result.results[0].category, "supplement-guides");
    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_guide() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/guides/biotin-for-hair/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_guide_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let guide = client.get_guide("biotin-for-hair").await.unwrap();

    assert_eq!(guide.slug, "biotin-for-hair");
    assert_eq!(guide.title, "Biotin for Hair Growth: What the Research Says");
    assert!(!guide.content.is_empty());
    assert_eq!(guide.category, "supplement-guides");
    mock.assert_async().await;
}

// -- Condition backward compatibility (nested in EvidenceLink) ----------------

#[tokio::test]
async fn test_condition_minimal_in_evidence() {
    // Verify that the expanded Condition struct still deserializes
    // when nested inside EvidenceLink with only slug + name fields.
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/api/evidence/42/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_evidence_json())
        .create_async()
        .await;

    let client = client_for(&server);
    let evidence = client.get_evidence(42).await.unwrap();

    // Nested condition has only slug + name; other fields should default
    assert_eq!(evidence.condition.slug, "hair-loss");
    assert_eq!(evidence.condition.name, "Hair Loss");
    assert!(evidence.condition.description.is_empty());
    assert!(evidence.condition.symptoms.is_empty());
    assert!(!evidence.condition.is_featured);
    mock.assert_async().await;
}
