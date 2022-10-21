use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Engin Diri", version, long_about = None)]
/// A simple tui Trivy plugin written in Rust
pub struct Args {
    #[arg(short, long)]
    pub image_name: String,
}

