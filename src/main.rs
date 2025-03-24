use repeat::{Args,run};
use::clap::Parser;


fn main() {
    let  args = Args::parse();
    run(args);
}