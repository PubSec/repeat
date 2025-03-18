use::clap::Parser;
use std::process::Command;


#[derive(Parser,Debug)]
#[command(version = "1.0.0",about = "The program allows you to repeat a command",long_about = None)]
pub struct Args{
    /// Command name eg ls,cat...
    pub command_name:Vec<String>,

    /// Number of times the command will be executed
    #[arg(short,long, default_value_t = 1)]
    pub count:u32,

    /// Shutdown computer after command is executed
    #[arg(short,long,default_value_t = 'n')]
    pub shutdown: char,

    /// Reboots computer after command is executed
    #[arg(short,long,default_value_t = 'n')]
    pub reboot: char,
}

pub fn run (args: Args){
    let command = args.command_name;
    let count = args.count;
    let shutdown_value = args.shutdown;
    let reboot_value = args.reboot;
    execute(command, count);
}

fn execute(command: Vec<String>, count: u32){
    for i in 0..count{
        let outptut = String::from_utf8(Command::new(&command[0]).args(&command[1..command.len()]).output().unwrap().stdout).unwrap();
    }
}