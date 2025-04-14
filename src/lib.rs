use::clap::Parser;
use std::process::{self, Command};
use std::thread::sleep;
use std::time::Duration;


#[derive(Parser,Debug)]
#[command(version = "1.1.0",about = "The program allows you to repeat a command",long_about = None)]
pub struct Args{
    /// Command name eg ls,cat...
    command_name:Vec<String>,

    /// Number of times the command will be executed
    #[arg(short,long, default_value_t = 1)]
    count:u32,

    ///Sleep between every iteration
    #[arg(short='t',long,default_value_t=0)]
    sleep:u32,

    /// Shutdown computer after command is executed
    #[arg(short,long,default_value_t = 'n')]
    shutdown: char,

    /// Reboots computer after command is executed
    #[arg(short,long,default_value_t = 'n')]
    reboot: char,

    /// Print the output of the commands
    #[arg(short,long,action)]
    verbose:bool, 
}

pub fn run (args: Args){
    let command = args.command_name;
    let count = args.count;
    let sleep_duration = args.sleep;
    let shutdown_value = args.shutdown;
    let reboot_value = args.reboot;
    let print_verbosely= args.verbose;
    if command.len() == 0{
        eprintln!("Command Error: {:?} not found",command);
        process::exit(1)
    }else {
        if shutdown_value == 'y'{
            execute(command, count,sleep_duration,print_verbosely);
            let _ = Command::new("shutdown").spawn();
        }else if reboot_value == 'y' {
            execute(command, count,sleep_duration,print_verbosely);
            let _ = Command::new("reboot").spawn();
        }else if reboot_value == 'y' && shutdown_value == 'y'{
            eprintln!("Error: Both shutdown and reboot can't be set a together");
        }else{
	    execute(command,count,sleep_duration,print_verbosely)
	}
    }
}

fn execute(command: Vec<String>, count: u32, sleep_duration:u32,print_verbosely:bool){
    for _ in 0..count{
        if print_verbosely{
	    sleep(Duration::from_secs(sleep_duration.into()));
            let output =  Command::new(&command[0]).args(&command[1..command.len()]).output().unwrap_or_else(|err| {
                eprintln!("Command Error: {:?} not found or {err}",command);
                process::exit(1)
            });
            let output_string  = String::from_utf8(output.stdout).unwrap_or_else(|err|{
                eprintln!("Parse Error: Unable to parse command output or {err}");
                process::exit(1)
            });
            println!("{}", output_string)
        }else {
	    sleep(Duration::from_secs(sleep_duration.into()));
            let _ =  Command::new(&command[0]).args(&command[1..command.len()]).output().unwrap_or_else(|err| {
                eprintln!("Command Error: {:?} not found or {err}",command);
                process::exit(1)
            });
        }
    }
}
