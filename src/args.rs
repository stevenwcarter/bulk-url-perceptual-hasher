use clap::Parser;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(term_width = 0)] // Just to make testing across clap features easier
pub struct Args {
    #[arg(long)]
    pub debug: bool,

    #[arg(long)]
    pub input: String,
}
