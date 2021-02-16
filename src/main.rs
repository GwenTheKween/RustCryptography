extern crate clap;
use clap::{App, load_yaml};
use std::path::PathBuf;
use std::fs;
use std::io;

fn prepare_hash(args: &clap::ArgMatches){
    println!("so far so good");

    println!("{:?}",args.is_present("input"));
    //is there a file path given? If not, pass stdin along
    let input_file_name = args.value_of("input").unwrap();

    let i_file = fs::File::open(input_file_name);
}

fn interactive_mode(){
    println!("Interactive mode not yet supported, sorry");
}

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    //subcommand processing
    match args.subcommand_name(){
        Some("hash") => {
            match args.subcommand_matches("hash"){
                Some(sub_args) => prepare_hash(sub_args),
                None => println!("this should not have happened"),
            }
        }
        None => interactive_mode(),
        _ => {
            println!("Incorrect subcommand");
            return;
        }
    }
}
