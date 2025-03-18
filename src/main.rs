use std::process::Command;
use repeat::Args;
use::clap::Parser;


fn main() {

    let  args = Args::parse();
    repeat::run(args);
    // for _ in 0..args.count{
    //     if args.command_name.len() > 1{
    //     let output = String::from_utf8(Command::new(&args.command_name[0]).args(&args.command_name[1..args.command_name.len()]).output().unwrap().stdout).unwrap();
    //     println!("{}",&output)
    //     }else {
    //     let _ = Command::new(&args.command_name[0]).spawn();   
    //     }

    // if args.shutdown == 'y'{
    //     println!("Command finished shuting down now");
    //     let _output = Command::new("shutdown").arg("now").spawn().unwrap();

    //     }
    // }
}

