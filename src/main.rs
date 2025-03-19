use repeat::Args;
use::clap::Parser;


fn main() {
    let  args = Args::parse();
    repeat::run(args);
}

