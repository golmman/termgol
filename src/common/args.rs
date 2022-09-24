use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value_t = 8)]
    pub frames_per_second: u16,
}
