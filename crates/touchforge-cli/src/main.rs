use anyhow::Result;
use clap::{Parser, Subcommand};
use touchforge_profile::load_profile;

#[derive(Parser)]
#[command(name = "touchforge")]
#[command(about = "TouchForge command-line tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import {
        file: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import { file } => {
            let profile = load_profile(&file)?;

            println!("Profile: {}", profile.name);
            println!("Controls: {}", profile.controls.len());

            for control in profile.controls {
                println!(
                    "- {} at ({}, {}) scale {} -> {}",
                    control.control_type, control.x, control.y, control.scale, control.binding
                );
            }
        }
    }

    Ok(())
}
