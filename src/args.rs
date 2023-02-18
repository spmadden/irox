use clap::{Args, Parser, Subcommand};

// use shadow_rs::shadow;
// shadow!(build);

#[derive(Parser, Debug)]
#[command(author, 
    version, 
    about, 
    // long_version=build::CLAP_LONG_VERSION
)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub command : CliCommand
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum CliCommand {
    
    /// Displays information about the input file
    #[command(name="info")]
    DisplayInfo(InfoArgs),

    /// Runs the processing & extraction algorithms
    #[command(name="extract")]
    RunProcessing(RunArgs),

    /// Prints extra information about the build environment
    #[command(name="extra-version")]
    PrintBuildInfo,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct InfoArgs {
    /// Path to the input USGS Topo GeoPDF file to read.
    pub input_file : String,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct RunArgs {
    /// Path to the input USGS Topo GeoPDF file to read.
    pub input_file : String,
}

pub(crate) fn print_build_info() {
    // build::print_build_in()
}