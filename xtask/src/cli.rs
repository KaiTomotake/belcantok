#[derive(clap::Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(clap::Subcommand)]
pub enum SubCommands {
    #[command(about = "Run the test suite")]
    Test,

    #[command(about = "Run clippy and treat warnings as errors")]
    Clippy,

    #[command(about = "Check that all code is formatted with rustfmt")]
    FmtCheck,

    #[command(about = "Format the code with nightly-2026-10-01")]
    Fmt,

    #[command(about = "Run fmt-check, clippy, and test quietly, reporting only errors")]
    Verify,
}
