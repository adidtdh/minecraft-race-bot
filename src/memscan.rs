use sysinfo::{ProcessExt, System, SystemExt};
use std::io;


pub fn get_java_pid() -> i32 {
    let s = System::new_all();
    let processes = s.processes_by_name("java");

    println!("List of processes named java");

    for process in  processes{
        println!("{}: {}",process.name(), process.pid());
    }

    print!("Enter desired pid:");
    let mut guess = String::new();
 
    io::stdin().read_line(&mut guess).expect("failed to readline");

    return guess.parse().unwrap();
}



