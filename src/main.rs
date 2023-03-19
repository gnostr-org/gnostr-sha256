use std::env;
use std::process;
use std::io::{Result};

//time functions
extern crate time;
extern crate chrono;
use chrono::{DateTime,Utc};
use std::time::{SystemTime, UNIX_EPOCH};



#[cfg(debug_assertions)]
use std::path::PathBuf;

//shell commands


//lib.rs
use gnostr_sha256::Config;

//main.rs functions
fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
#[cfg(debug_assertions)]
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

//debug
#[cfg(debug_assertions)]

fn example() {

    //println!("Debugging enabled");
    //println!("cwd={:?}",get_current_working_dir());

}
#[cfg(not(debug_assertions))]
fn example() {

    //println!("Debugging disabled");
    //println!("cwd={:?}",get_current_working_dir());

}

fn main() -> Result<()> {

//#[cfg(debug_assertions)]
//    println!("Debugging enabled");
//
//#[cfg(not(debug_assertions))]
//    //println!("Debugging disabled");

let start = time::get_time();
#[cfg(debug_assertions)]
        println!("start={:#?}", start);

let _epoch = get_epoch_ms();
let _system_time = SystemTime::now();
let _datetime: DateTime<Utc> = _system_time.into();

#[cfg(debug_assertions)]
        let cwd = get_current_working_dir();
#[cfg(debug_assertions)]
        println!("cwd={:#?}", cwd);

let args: Vec<String> = env::args().collect();
let _dirname = &args[0];

if cfg!(debug_assertions) {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");
} else {
    //#[cfg(not(debug_assertions))]
    //println!("Debugging disabled");
}

example();


let config = Config::build(&args).unwrap_or_else(|_err| {
    println!("Usage: gnostr-sha256 <string>");
    process::exit(0);
});

//println!("{}", config.query);

if let Err(e) = gnostr_sha256::run(config) {
    println!("Application error: {e}");
    process::exit(1);
}



let _duration = time::get_time() - start;
Ok(())
}//end main
