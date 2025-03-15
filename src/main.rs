use std::process::Command;
use::clap::Parser;

fn main() {

    let  args = Args::parse();
    for _ in 0..args.count{
        
        let _ = Command::new(&args.command_name[0]).spawn();


    if args.shutdown == 'y'{
        println!("Command finished shutdowning down now");
        let _output = Command::new("shutdown").output();

        }
    }
}

#[derive(Parser,Debug)]
#[command(version = "1.0.0",about = "The program allows you to repeat a command",long_about = None)]
struct Args{

    /// Command name eg ls,cat...
    command_name:Vec<String>,

    /// Number of times the command will be executed
    #[arg(short,long, default_value_t = 1)]
    count:u8,

    /// Shutdown computer after command is executed
    #[arg(short,long,default_value_t = 'n')]
    shutdown: char,
}