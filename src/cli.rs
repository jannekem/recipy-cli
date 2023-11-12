use clap::Parser;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// The URL to fetch
    #[clap(required_unless_present = "init")]
    pub url: Option<String>,

    /// Filename
    #[clap(short, long)]
    pub filename: Option<String>,

    /// Force overwrite
    #[clap(long)]
    pub force: bool,

    /// Add to favorites
    #[clap(long)]
    pub favorite: bool,

    /// Oven temperature
    #[clap(long)]
    pub temperature: Option<String>,

    /// Print recipe to stdout
    #[clap(long)]
    pub stdout: bool,

    /// Initialize directory structure
    #[clap(long)]
    pub init: bool,
}
