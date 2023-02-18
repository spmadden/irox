mod args;
mod info;
mod open;
mod pdf;
mod tabs;

use args::{CliArgs, CliCommand};
use clap::Parser;


fn main() {
    let args = CliArgs::parse();

    match args.command {
        CliCommand::PrintBuildInfo => {
            args::print_build_info();
            return;
        },
        CliCommand::DisplayInfo(info) => info::print_pdf_info(&info.input_file),
        CliCommand::RunProcessing(_) => todo!(),
    }
    
}

