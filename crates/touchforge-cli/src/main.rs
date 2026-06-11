use anyhow::Result;
use clap::{Parser, Subcommand};

use touchforge_profile::{
    export_json,
    load_icp,
    load_profile,
    save_profile,
    TfProfile,
};

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

    CreateProfile {
        name: String,
    },

    OpenProfile {
        file: String,
    },

    Export {
        input: String,
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import { file } => {
            let profile = load_icp(&file)?;

            println!("Profile: {}", profile.name);
            println!("Cursor Speed: {}", profile.cursor_speed);
            println!("Elements: {}", profile.elements.len());
        }

        Commands::CreateProfile { name } => {
            let profile = TfProfile::new(name.clone());

            let filename = format!("{}.tfp", name);

            save_profile(&profile, &filename)?;

            println!("Created {}", filename);
        }

        Commands::OpenProfile { file } => {
            let profile = load_profile(&file)?;

            println!("Profile: {}", profile.name);
            println!("Version: {}", profile.version);
            println!("Elements: {}", profile.elements.len());
        }

        Commands::Export { input, output } => {
            let profile = load_profile(&input)?;

            export_json(&profile, &output)?;

            println!("Exported {} -> {}", input, output);
        }
    }

    Ok(())
}
