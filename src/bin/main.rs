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
    Get(GetWorkspace),
}

#[derive(Args)]
struct CreateWorkSpace {
    name: Option<String>,
    path: Option<String>,
}

#[derive(Args)]
struct GetWorkspace {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(name) => {
            let path_name = name.name.clone().unwrap();
            let path = name.path.clone().unwrap();
            ph::manage::workspace::create(path_name, path)
        }
        Commands::Get(input) => {
            let name = input.name.clone().unwrap();
            ph::manage::workspace::get(name).unwrap()
        }
    }
}
