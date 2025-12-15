use clap::Parser;
use std::path::PathBuf;
use syster_cli::run_analysis;

#[derive(Parser)]
#[command(name = "syster")]
#[command(about = "SysML v2 parser and semantic analyzer", long_about = None)]
struct Cli {
    /// Input file or directory to analyze
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Skip loading standard library
    #[arg(long)]
    no_stdlib: bool,

    /// Path to custom standard library (default: sysml.library)
    #[arg(long, value_name = "PATH")]
    stdlib_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let result = run_analysis(
        &cli.input,
        cli.verbose,
        !cli.no_stdlib,
        cli.stdlib_path.as_ref(),
    )
    .map_err(|e| anyhow::anyhow!(e))?;

    println!(
        "✓ Successfully analyzed {} files ({} symbols)",
        result.file_count, result.symbol_count
    );

    Ok(())
}
