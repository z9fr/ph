extern crate ph;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new workspace
    Create(CreateWorkSpace),
}

#[derive(Args)]
struct CreateWorkSpace {
    name: Option<String>,
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    ph::add(12, 13);

    match &cli.command {
        Commands::Create(name) => {
            println!(
                "create new workspace {:?} the workspace path is {:?}",
                name.name, name.path
            );
        }
    }
}
