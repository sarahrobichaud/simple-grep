use io_project::{Haystack, search};
use std::{env, error, process};

struct Configuration<'a> {
    needle: &'a String,
    file_path: &'a String
}

impl<'a> Configuration<'a> {
    fn build(args: &'a [String]) -> Result<Configuration<'a>, &'static str> {

        if args.len() < 2 {
            return Err("No arguments provided.");
        }

        let Some(file_path) = args.get(2) else {
            return Err("No file path provided.");
        };

        let query = args.get(1).expect("This should always be set if file_path is readable!");

        Ok(Configuration {
            needle: query,
            file_path
        })
    }

    fn print(&self) {
        println!("Searching for {}", self.needle);
        println!("In file: {}", self.file_path);
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Configuration::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1)
    };
}

fn run(config: Configuration) -> Result<(), Box<dyn error::Error>>{
    config.print();

    print_divider();

    let haystack = Haystack::build_from_file(&config.file_path)?;

    haystack.print();

    print_divider();

    let results = search(&config.needle, &haystack);

    if results.len() <= 0 {
        println!("No results for {}", config.needle);
    }else {
        for match_result in search(&config.needle, &haystack) {
            match_result.print()
        }
    }

    Ok(())
}

fn print_divider() {
    println!("-----------------------");
}
