use clap::Parser;

/// Simple, Fast, and Efficient Launcher for RotMG Exalt.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Username
    #[clap(short, long, value_parser)]
    pub username: String,

    /// Password
    #[clap(short, long, value_parser)]
    pub password: String,
}