use std::path::PathBuf;
use clap::Parser;
use fugu::analyzer::file_system_analyzer::analyze;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    path: PathBuf
}

pub fn run() {
    let args = Args::parse();
    
    let size = analyze(args.path);
    println!("{}", size);
}