use clap::{Parser, Subcommand};
use tarmac_score::DimensionScorer;

#[derive(Parser)]
#[command(name = "tarmac")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Corpus {
        path: std::path::PathBuf,
    },
    Score {
        path: std::path::PathBuf,
    },
    TierSla {
        path: std::path::PathBuf,
    },
    Gap {
        #[arg(long)]
        scale: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Corpus { path } => {
            let content = std::fs::read_to_string(path)?;
            let entry = tarmac_corpus::CorpusEntry::from_markdown(&content)?;
            println!("{:?} {:?}", entry.id, entry.validate());
        }
        Commands::Score { path } => {
            let content = std::fs::read_to_string(path)?;
            let entry = tarmac_corpus::CorpusEntry::from_markdown(&content)?;
            let scorer = tarmac_score::ProvisionalScorer::default();
            for dimension in tarmac_score::Dimension::all() {
                let score = scorer.score(&entry, dimension);
                println!("{} {}", dimension.code(), score.value());
            }
        }
        Commands::TierSla { path } => {
            let content = std::fs::read_to_string(path)?;
            let entry = tarmac_corpus::CorpusEntry::from_markdown(&content)?;
            println!(
                "{:?} {}",
                tarmac_tier::classify(&entry),
                tarmac_tier::tier_sla_gap(&entry).is_some()
            );
        }
        Commands::Gap { scale } => {
            let scale = tarmac_corpus::Scale::parse(&scale).ok_or("invalid scale")?;
            let result = tarmac_gap::find_gaps(&[], &tarmac_score::Rubric::v0(), scale, &[], false);
            println!("{} {}", result.null_result, result.regions.len());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_gap() {
        assert!(Cli::try_parse_from(["tarmac", "gap", "--scale", "national"]).is_ok());
    }

    #[test]
    fn parses_corpus() {
        assert!(Cli::try_parse_from(["tarmac", "corpus", "some.md"]).is_ok());
    }
}
