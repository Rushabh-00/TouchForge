use anyhow::Result;
use clap::{Parser, Subcommand};

use touchforge_profile::load_icp;

#[derive(Parser)]
#[command(name = "touchforge")]
#[command(about = "TouchForge command-line tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Import { file: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import { file } => {
            let profile = load_icp(&file)?;

            println!("Profile: {}", profile.name);
            println!("Cursor Speed: {}", profile.cursor_speed);
            println!("Elements: {}", profile.elements.len());

            println!();

            for element in profile.elements {
    println!("{} ({}, {})", element.element_type, element.x, element.y);
}
            }
        }
    }

    Ok(())
}
