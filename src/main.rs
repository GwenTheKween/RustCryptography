extern crate clap;
use clap::{App, load_yaml};
use std::path::PathBuf;

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    //argv processing
    let mut wrong = false; //flag to tell if an argument has been misused

    //parse input file
    let file_arg = args.value_of("input file");
    let mut ifile = PathBuf::new();
    match file_arg{
        None => wrong = true,
        Some(_) => {
            ifile = PathBuf::from(file_arg.unwrap());
        }
    }

    //done parsing arguments, now encrypt
    if wrong {
        println!("Usage without command line arguments is not yet supported. Please use command line arguments (-h for help)");
    }else{
        println!("the chosen file is {}",file_arg.unwrap());
    }
}
