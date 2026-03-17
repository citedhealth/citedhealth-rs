//! # citedhealth
//!
//! Async Rust client for the [Cited Health](https://citedhealth.com) REST API.
//!
//! Access evidence-based supplement research data including ingredients,
//! conditions, evidence links with grades (A-F), and PubMed-indexed papers.
//!
//! ## Quick Start
//!
//! ```no_run
//! use citedhealth::CitedHealth;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), citedhealth::CitedHealthError> {
//!     let client = CitedHealth::new();
//!
//!     // Search for ingredients
//!     let ingredients = client.list_ingredients(Some("biotin"), None).await?;
//!     println!("Found {} ingredients", ingredients.count);
//!
//!     // Get evidence for a specific ingredient-condition pair
//!     let evidence = client.list_evidence(Some("biotin"), Some("hair-loss")).await?;
//!     for link in &evidence.results {
//!         println!("{} for {}: grade {}", link.ingredient.name, link.condition.name, link.grade);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod types;

pub use client::{CitedHealth, CitedHealthBuilder};
pub use error::CitedHealthError;
pub use types::*;
