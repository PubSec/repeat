use std::process::Command;
use::clap::Parser;

fn main() {

    let args = Args::parse();
    for _ in 0..args.count{
        Command::new(&args.command_name).output().expect("An error");
    }
}

#[derive(Parser,Debug)]
#[command(version = "1.0.0",about = "The program allows you to repeat a command",long_about = None)]
struct Args{
    #[arg(short='n',long="command-name")]
    command_name:String,

    #[arg(short,long, default_value_t = 1)]
    count:u8,

    #[arg(short,long,default_value_t = 'n')]
    shutdown: char,
}