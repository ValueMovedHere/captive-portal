use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = 8080u16)]
    pub port: u16,
}
