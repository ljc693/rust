use std::env;
use std::process;
use minigrep::Config;

fn main(){
    let args:Vec<String> = env::args().collect();

    println!("{:?}",args);

    // let query = &args[1];
    // let filename = &args[2];
    //println!("{0},{1}",query,filename);

    // let config = Config::build(&args).unwrap_or_else(op);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e)= minigrep::run(config)
    {
        eprintln!("Application error:{}",e);
        process::exit(1);
    };


}
