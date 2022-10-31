mod pbf;
mod osm;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use log::LevelFilter;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Convert {
        #[clap(short, long)]
        file: PathBuf,
    
        #[clap(short, long)]
        outdir: String,
    },

    /// Displays info about the selected file
    Info {
        #[clap(short, long)]
        file: PathBuf
    }
}

fn info(file: PathBuf) {
    osm::OSMFile::scan(file.as_path())
}

fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();
    let args = Args::parse();
    

    match args.command {
        Commands::Convert { file, outdir } => todo!(),
        Commands::Info { file } => info(file),
    }
}
