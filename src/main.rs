mod init;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "workctl",
    about = "workctl command line interface",
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the workctl version
    Version,
    /// Set up the local environment (downloads a python-build-standalone distribution)
    Init(InitArgs),
}

#[derive(Args)]
struct InitArgs {
    /// Python version to install: a prefix like "3.14" (newest patch) or exact like "3.14.10"
    #[arg(long, default_value = init::DEFAULT_PYTHON_VERSION)]
    python_version: String,

    /// Exact build options to require, e.g. "pgo+lto" or "freethreaded+pgo+lto".
    /// Default: freethreaded+pgo+lto, falling back to the best available build
    #[arg(long)]
    variant: Option<String>,

    /// python-build-standalone release tag, e.g. "20261026" (default: latest)
    #[arg(long)]
    release: Option<String>,
}

fn version_string() -> String {
    format!("workctl {}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Version => {
            println!("{}", version_string());
            Ok(())
        }
        Command::Init(args) => init::run(&init::Options {
            python_version: args.python_version,
            variant: args.variant,
            release: args.release,
        }),
    };
    if let Err(err) = result {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_has_name_and_version() {
        assert_eq!(
            version_string(),
            concat!("workctl ", env!("CARGO_PKG_VERSION"))
        );
    }
}
