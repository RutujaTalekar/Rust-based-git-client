#[macro_use]
extern crate lazy_static;

mod add;
mod commit;
mod error;
mod file;
mod index;
mod init;
mod tree;

use clap::{App, Arg, SubCommand};

fn main() {
    let m = App::new("GitClient")
        .subcommand(SubCommand::with_name("init").about("Initializes the repository"))
        .subcommand(
            SubCommand::with_name("add").about("Add a file").arg(
                Arg::with_name("file")
                    .help("File to add")
                    .index(1)
                    .multiple(true)
                    .required(true),
            ),
        ).subcommand(SubCommand::with_name("commit").about("commits a change"))
        .get_matches();

    match m.subcommand() {
        ("init", Some(..)) => match init::init() {
            Ok(()) => println!("Repository initialized"),
            Err(..) => println!("Already initialized"),
        }
        
        _ => println!("Command not recognized."),
    }
}