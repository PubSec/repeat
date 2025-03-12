use std::env;


fn main() {
    println!("The length of the args {}", env::args().len() - 1);
    //for arg in env::args().next() {
	//let command  = arg;
	//println!("Command is {}",command)
    //}
    // println!("Here are the args {:?x}", env::args())
//    if let Some(arg) = env::args().next(){
	//println!("Progaram name is {}",arg);
	//env::args().next();
	//   if let Some(arg) = env::args().next(){
	//	println!("The command name is {}", arg)
//	    }
    //    }

    let arguments:Vec<String> = env::args().collect::<Vec<_>>();
    let program_name = env::args().next();
    //let command = ;
    println!("{:?}",arguments);
    //println!("{:?}",command)
}
