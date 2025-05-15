use anyhow::Result;
use clap::{Parser, Subcommand};

mod features;

use features::{analyzer, history, stats, suggester, writer};

#[derive(Parser)]
#[command(name = "alrc", about = "CLI utility for generating shell aliases")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        /// Specify shell (bash, zsh)
        #[arg(short, long)]
        shell: Option<String>,
    },
    Suggest,
    Add {
        indices: Vec<usize>,
        /// Shell for which to add (bash, zsh)
        #[arg(short, long)]
        shell: Option<String>,
    },
    Stats,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze { shell } => {
            let history = history::read_history(shell)?;
            let groups = analyzer::analyze(&history);
            suggester::display_groups(&groups);
        }
        Commands::Suggest => {
            let history = history::read_history(None)?;
            let suggestions = suggester::suggest(&history);
            suggester::interactive_add(&suggestions)?;
        }
        Commands::Add { indices, shell } => {
            writer::add_aliases(indices, shell)?;
        }
        Commands::Stats => {
            let history = history::read_history(None)?;
            stats::show(&history)?;
        }
    }
    Ok(())
}
