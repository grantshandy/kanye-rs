use clap::{crate_version, crate_name, App};
use colored::Colorize;

fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Says Kanye Quotes")
        .get_matches();
    
    let quote = match kanye::quote() {
        Ok(quote) => quote,
        Err(error) => {
            eprintln!("{} {}\n", "error:".red().bold(), error);
            eprintln!("USAGE:\n    kanye\n");
            eprintln!("For more information try {}", "--help".green());
            std::process::exit(1);
        }
    };

    println!("Kanye says \"{}\"", quote);
}