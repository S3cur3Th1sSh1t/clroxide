use clroxide::clr::Clr;
use std::{env, fs, process::exit};

fn main() -> Result<(), String> {
    let (path, args) = prepare_args();

    let contents = fs::read(path).expect("Unable to read file");
    let mut clr = Clr::new(contents, args)?;

    // We are not redirecting the output, so that user interactive assemblies
    // also work properly, without the program hanging and waiting for final output.
    let _ = clr.run_no_redirect()?;

    Ok(())
}

fn prepare_args() -> (String, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a path to a dotnet executable");

        exit(1)
    }

    let mut command_args: Vec<String> = vec![];

    if args.len() > 2 {
        command_args = args.split_off(2)
    }

    let path = args[1].clone();

    println!("[+] Running `{}` with given args: {:?}", path, command_args);

    return (path, command_args);
}
