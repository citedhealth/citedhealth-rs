# citedhealth

[![Crates.io version](https://agentgif.com/badge/crates/citedhealth/version.svg)](https://crates.io/crates/citedhealth)
[![docs.rs](https://docs.rs/citedhealth/badge.svg)](https://docs.rs/citedhealth)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://agentgif.com/badge/github/citedhealth/citedhealth-rs/stars.svg)](https://github.com/citedhealth/citedhealth-rs)

Async Rust client for the [Cited Health](https://citedhealth.com) REST API -- evidence-based supplement research data, PubMed papers, and evidence grades.

Cited Health aggregates clinical research on supplements and health ingredients across 6 sites (citedhealth.com, haircited.com, sleepcited.com, gutcited.com, immunecited.com, braincited.com). The API provides access to 188 ingredients, 84 conditions, 323 evidence links with grades (A-F), 6,197 PubMed-indexed papers, 228 glossary terms, and 50 guides. Every evidence link is backed by study counts, participant totals, and effect direction.

> **Explore the data at [citedhealth.com](https://citedhealth.com)** -- [Ingredients](https://citedhealth.com/ingredients/), [Evidence](https://citedhealth.com/api/evidence/), [Papers](https://citedhealth.com/papers/), [Conditions](https://citedhealth.com/conditions/), [Glossary](https://citedhealth.com/glossary/), [Guides](https://citedhealth.com/guides/)

<p align="center">
  <a href="https://agentgif.com/s6D4nzk9"><img src="https://media.agentgif.com/s6D4nzk9.gif" alt="citedhealth Rust CLI demo — search ingredients, evidence grades, and PubMed papers" width="800"></a>
</p>

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [Command-Line Interface](#command-line-interface)
- [What You Can Do](#what-you-can-do)
  - [Search Ingredients](#search-ingredients)
  - [Lookup Evidence Grades](#lookup-evidence-grades)
  - [Search PubMed Papers](#search-pubmed-papers)
  - [Browse Conditions](#browse-conditions)
  - [Glossary Terms](#glossary-terms)
  - [Educational Guides](#educational-guides)
- [API Reference](#api-reference)
- [Error Handling](#error-handling)
- [Custom Configuration](#custom-configuration)
- [Learn More About Evidence-Based Supplements](#learn-more-about-evidence-based-supplements)
- [Also Available](#also-available)
- [License](#license)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
citedhealth = "0.4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Or install the CLI directly:

```bash
cargo install citedhealth
```

## Quick Start

```rust
use citedhealth::CitedHealth;

#[tokio::main]
async fn main() -> Result<(), citedhealth::CitedHealthError> {
    let client = CitedHealth::new();

    // Search for supplement ingredients
    let ingredients = client.list_ingredients(Some("biotin"), None).await?;
    println!("Found {} ingredients", ingredients.count);

    // Get evidence for biotin and hair loss
    let evidence = client.list_evidence(Some("biotin"), Some("nutritional-deficiency-hair-loss")).await?;
    for link in &evidence.results {
        println!(
            "{} for {}: grade {} ({} studies, {} participants)",
            link.ingredient.name,
            link.condition.name,
            link.grade,
            link.total_studies,
            link.total_participants,
        );
    }

    // Search PubMed papers
    let papers = client.list_papers(Some("melatonin sleep"), None).await?;
    for paper in &papers.results {
        println!("{} ({}) - {}", paper.title, paper.publication_year.unwrap_or(0), paper.journal);
    }

    Ok(())
}
```

## Command-Line Interface

Install the CLI with `cargo install citedhealth`, then query supplement research data directly from the terminal.

```bash
# Search ingredients by name
citedhealth ingredients biotin

# Filter ingredients by category
citedhealth ingredients --category vitamins

# Get a single ingredient by slug
citedhealth ingredient biotin

# Look up evidence for an ingredient-condition pair
citedhealth evidence biotin nutritional-deficiency-hair-loss

# Search PubMed papers by keyword and year
citedhealth papers melatonin --year 2024

# Get a single paper by PMID
citedhealth paper 35959711

# Search health conditions
citedhealth conditions "hair loss"

# Get a single condition by slug
citedhealth condition hair-loss

# Search glossary terms
citedhealth glossary "bioavailability"

# Get a single glossary term by slug
citedhealth glossary-term rct

# Search educational guides
citedhealth guides "biotin"

# Get a single guide by slug
citedhealth guide biotin-for-hair

# Compact JSON output (default is pretty-printed)
citedhealth ingredients biotin --json
```

| Command | Description |
|---------|-------------|
| `ingredients [query]` | List or search ingredients. `-c, --category` to filter. |
| `ingredient <slug>` | Get a single ingredient by slug. |
| `evidence <ingredient> <condition>` | Look up evidence for an ingredient-condition pair. |
| `papers [query]` | List or search PubMed papers. `-y, --year` to filter. |
| `paper <pmid>` | Get a single paper by PubMed ID. |
| `conditions [query]` | List or search health conditions. |
| `condition <slug>` | Get a single condition by slug. |
| `glossary [query]` | List or search glossary terms. |
| `glossary-term <slug>` | Get a single glossary term by slug. |
| `guides [query]` | List or search educational guides. |
| `guide <slug>` | Get a single guide by slug. |

All commands accept `--json` for compact (single-line) JSON output.

## What You Can Do

### Search Ingredients

Query supplement ingredients by name or category. Each ingredient includes mechanism of action, recommended dosage, available forms, and featured status.

| Category | Examples |
|----------|---------|
| Vitamins | Biotin, Vitamin D, Vitamin C, B12 |
| Minerals | Zinc, Magnesium, Iron, Selenium |
| Herbs | Ashwagandha, Saw Palmetto, Ginkgo |
| Amino Acids | L-Theanine, NAC, Creatine |

```rust
// Search by name
let results = client.list_ingredients(Some("vitamin d"), None).await?;

// Filter by category
let herbs = client.list_ingredients(None, Some("herbs")).await?;

// Get a specific ingredient with full details
let biotin = client.get_ingredient("biotin").await?;
println!("Forms: {:?}", biotin.forms);           // ["capsule", "tablet", "liquid"]
println!("Mechanism: {}", biotin.mechanism);      // Coenzyme for carboxylase enzymes...
```

Learn more: [Browse Ingredients](https://citedhealth.com/) | [Evidence Database](https://citedhealth.com/api/evidence/) | [API Docs](https://citedhealth.com/developers/)

### Lookup Evidence Grades

Evidence links represent the research relationship between an ingredient and a health condition. Each link includes an evidence grade (A-F), study count, participant total, and effect direction.

| Grade | Label | Meaning |
|-------|-------|---------|
| A | Strong Evidence | Consistent results from multiple high-quality RCTs |
| B | Good Evidence | Positive results from well-designed studies |
| C | Moderate Evidence | Some positive findings but inconsistent |
| D | Weak Evidence | Limited or preliminary evidence |
| F | No Evidence | No significant benefit demonstrated |

```rust
// Find evidence for biotin and hair loss
let evidence = client.list_evidence(Some("biotin"), Some("nutritional-deficiency-hair-loss")).await?;
if let Some(link) = evidence.results.first() {
    println!("Grade: {} ({})", link.grade, link.grade_label);
    println!("Direction: {}", link.direction);
    println!("Studies: {}, Participants: {}", link.total_studies, link.total_participants);
}

// Get a specific evidence link by ID
let link = client.get_evidence(42).await?;
```

Learn more: [Evidence Database](https://citedhealth.com/api/evidence/) | [Grading Methodology](https://citedhealth.com/editorial-policy/) | [Hair Health](https://haircited.com) | [Sleep Health](https://sleepcited.com)

### Search PubMed Papers

Access 6,197 PubMed-indexed papers with metadata including journal, study type, citation count, and open access status.

```rust
// Search by title keywords
let papers = client.list_papers(Some("melatonin"), None).await?;

// Filter by publication year
let recent = client.list_papers(Some("ashwagandha"), Some(2024)).await?;

// Get a specific paper by PMID
let paper = client.get_paper("34567890").await?;
println!("{} - {} ({})", paper.title, paper.journal, paper.study_type);
println!("Open access: {}", paper.is_open_access);
```

Learn more: [Paper Database](https://citedhealth.com/papers/) | [OpenAPI Spec](https://citedhealth.com/api/openapi.json) | [PubMed](https://pubmed.ncbi.nlm.nih.gov/)

### Browse Conditions

Explore 84 health conditions across 6 specialized sites -- hair health, sleep, gut, immune, and brain. Each condition includes prevalence data, symptoms, risk factors, and linked evidence.

```rust
// List all conditions
let conditions = client.list_conditions(None).await?;
println!("Total conditions: {}", conditions.count);

// Search conditions by name
let results = client.list_conditions(Some("hair loss")).await?;

// Get a single condition with full details
let condition = client.get_condition("hair-loss").await?;
println!("{}: {}", condition.name, condition.description);
println!("Symptoms: {:?}", condition.symptoms);
```

Learn more: [Conditions Database](https://citedhealth.com/conditions/) | [Hair Health](https://haircited.com) | [Sleep Health](https://sleepcited.com) | [Gut Health](https://gutcited.com)

### Glossary Terms

Access 228 glossary terms covering research methodology, nutrients, biological processes, and health conditions. Each term includes a short definition, full definition, and optional abbreviation.

```rust
// List all glossary terms
let terms = client.list_glossary(None).await?;

// Search by keyword
let results = client.list_glossary(Some("bioavailability")).await?;

// Get a specific term
let term = client.get_glossary_term("rct").await?;
println!("{} ({}): {}", term.term, term.abbreviation, term.short_definition);
```

Learn more: [Glossary](https://citedhealth.com/glossary/) | [Editorial Policy](https://citedhealth.com/editorial-policy/) | [API Docs](https://citedhealth.com/developers/)

### Educational Guides

Browse 50 in-depth guides on supplement research, ingredient deep-dives, and health condition explainers.

```rust
// List all guides
let guides = client.list_guides(None).await?;
println!("Total guides: {}", guides.count);

// Search guides by title
let results = client.list_guides(Some("biotin")).await?;

// Get a single guide with full content
let guide = client.get_guide("biotin-for-hair").await?;
println!("{}: {}", guide.title, guide.meta_description);
```

Learn more: [All Guides](https://citedhealth.com/guides/) | [Immune Health](https://immunecited.com) | [Brain Health](https://braincited.com)

## API Reference

| Method | Description |
|--------|-------------|
| `list_ingredients(q, category)` | List ingredients with optional search and category filter |
| `get_ingredient(slug)` | Get a single ingredient by slug |
| `list_evidence(ingredient, condition)` | List evidence links with optional filters |
| `get_evidence(id)` | Get a single evidence link by ID |
| `list_papers(q, year)` | List papers with optional search and year filter |
| `get_paper(pmid)` | Get a single paper by PubMed ID |
| `list_conditions(q)` | List conditions with optional search |
| `get_condition(slug)` | Get a single condition by slug |
| `list_glossary(q)` | List glossary terms with optional search |
| `get_glossary_term(slug)` | Get a single glossary term by slug |
| `list_guides(q)` | List guides with optional search |
| `get_guide(slug)` | Get a single guide by slug |

All methods are async and return `Result<T, CitedHealthError>`.

## Error Handling

The client returns typed errors for different failure modes:

```rust
use citedhealth::{CitedHealth, CitedHealthError};

let client = CitedHealth::new();
match client.get_ingredient("nonexistent").await {
    Ok(ingredient) => println!("Found: {}", ingredient.name),
    Err(CitedHealthError::NotFound { resource }) => {
        println!("Not found: {}", resource);
    }
    Err(CitedHealthError::RateLimit { retry_after }) => {
        println!("Rate limited, retry after {}s", retry_after);
    }
    Err(CitedHealthError::Api { status, message }) => {
        println!("API error {}: {}", status, message);
    }
    Err(CitedHealthError::Http(e)) => {
        println!("Network error: {}", e);
    }
}
```

## Custom Configuration

Use the builder for custom base URL and timeout:

```rust
use std::time::Duration;
use citedhealth::CitedHealth;

let client = CitedHealth::builder()
    .base_url("https://staging.citedhealth.com")
    .timeout(Duration::from_secs(60))
    .build()?;
```

## Learn More About Evidence-Based Supplements

- **Tools**: [Evidence Checker](https://citedhealth.com/api/evidence/) · [Ingredient Browser](https://citedhealth.com/) · [Paper Search](https://citedhealth.com/papers/)
- **Browse**: [Hair Health](https://haircited.com) · [Sleep Health](https://sleepcited.com) · [Gut Health](https://gutcited.com) · [Immune Health](https://immunecited.com) · [Brain Health](https://braincited.com)
- **Guides**: [Grading Methodology](https://citedhealth.com/editorial-policy/) · [Medical Disclaimer](https://citedhealth.com/medical-disclaimer/)
- **API**: [REST API Docs](https://citedhealth.com/developers/) · [OpenAPI Spec](https://citedhealth.com/api/openapi.json)
- **Python**: [citedhealth on PyPI](https://pypi.org/project/citedhealth/)
- **TypeScript**: [citedhealth on npm](https://www.npmjs.com/package/citedhealth)
- **Go**: [citedhealth-go on pkg.go.dev](https://pkg.go.dev/github.com/citedhealth/citedhealth-go)
- **Ruby**: [citedhealth on RubyGems](https://rubygems.org/gems/citedhealth)

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **PyPI** | `pip install citedhealth` | [PyPI](https://pypi.org/project/citedhealth/) |
| **npm** | `npm install citedhealth` | [npm](https://www.npmjs.com/package/citedhealth) |
| **Go** | `go get github.com/citedhealth/citedhealth-go` | [pkg.go.dev](https://pkg.go.dev/github.com/citedhealth/citedhealth-go) |
| **Ruby** | `gem install citedhealth` | [RubyGems](https://rubygems.org/gems/citedhealth) |
| **MCP** | `uvx citedhealth-mcp` | [PyPI](https://pypi.org/project/citedhealth-mcp/) |

## License

MIT
