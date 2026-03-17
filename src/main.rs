use std::process;

use clap::{Parser, Subcommand};
use citedhealth::CitedHealth;

/// Cited Health CLI -- evidence-based supplement research from the terminal.
///
/// Access ingredients, evidence grades, and PubMed papers via the
/// Cited Health REST API (https://citedhealth.com).
#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    /// Output compact JSON instead of pretty-printed.
    #[clap(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List or search supplement ingredients.
    Ingredients {
        /// Search by name (case-insensitive).
        query: Option<String>,

        /// Filter by category (e.g. "vitamins", "herbs", "amino-acids").
        #[clap(short, long)]
        category: Option<String>,
    },

    /// Get a single ingredient by slug.
    Ingredient {
        /// Ingredient slug (e.g. "biotin", "vitamin-d").
        slug: String,
    },

    /// Look up evidence for an ingredient-condition pair.
    Evidence {
        /// Ingredient slug (e.g. "biotin").
        ingredient: String,

        /// Condition slug (e.g. "hair-loss").
        condition: String,
    },

    /// List or search PubMed-indexed papers.
    Papers {
        /// Search in title (case-insensitive).
        query: Option<String>,

        /// Filter by publication year.
        #[clap(short, long)]
        year: Option<u32>,
    },

    /// Get a single paper by PubMed ID.
    Paper {
        /// PubMed identifier (PMID).
        pmid: String,
    },
}

fn print_json<T: serde::Serialize>(value: &T, compact: bool) {
    let output = if compact {
        serde_json::to_string(value)
    } else {
        serde_json::to_string_pretty(value)
    };
    match output {
        Ok(s) => println!("{s}"),
        Err(e) => {
            eprintln!("Error serializing JSON: {e}");
            process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = CitedHealth::new();

    let result = run(&client, &cli).await;
    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

async fn run(client: &CitedHealth, cli: &Cli) -> Result<(), citedhealth::CitedHealthError> {
    match &cli.command {
        Commands::Ingredients { query, category } => {
            let resp = client
                .list_ingredients(query.as_deref(), category.as_deref())
                .await?;
            print_json(&resp, cli.json);
        }
        Commands::Ingredient { slug } => {
            let ingredient = client.get_ingredient(slug).await?;
            print_json(&ingredient, cli.json);
        }
        Commands::Evidence {
            ingredient,
            condition,
        } => {
            let resp = client
                .list_evidence(Some(ingredient.as_str()), Some(condition.as_str()))
                .await?;
            if let Some(link) = resp.results.first() {
                print_json(link, cli.json);
            } else {
                eprintln!("No evidence found for {ingredient} + {condition}");
                process::exit(1);
            }
        }
        Commands::Papers { query, year } => {
            let resp = client.list_papers(query.as_deref(), *year).await?;
            print_json(&resp, cli.json);
        }
        Commands::Paper { pmid } => {
            let paper = client.get_paper(pmid).await?;
            print_json(&paper, cli.json);
        }
    }
    Ok(())
}
