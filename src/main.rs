use io_project::{Haystack, SearchOptions, print_search_result, search};
use std::{env, error, process};

mod helper;

struct Configuration<'a> {
    needle: &'a str,
    file_path: &'a str,
    is_verbose: bool,
    options: SearchOptions,
}

impl<'a> Configuration<'a> {
    fn build(args: &'a [String]) -> Result<Configuration<'a>, &'static str> {
        let Some(needle) = args.get(1) else {
            return Err("No query provided. Usage: io_project <NEEDLE> <FILE_PATH>");
        };

        let Some(file_path) = args.get(2) else {
            return Err("No file path provided.");
        };

        let ignore_case = helper::env_bool("IO_PROJECT_IGNORE_CASE", false);
        let is_verbose = helper::env_bool("IO_PROJECT_IS_VERBOSE", false);

        Ok(Configuration {
            needle,
            file_path,
            is_verbose,
            options: SearchOptions::build(ignore_case),
        })
    }

    fn print(&self) {
        println!("Searching for {}", self.needle);
        println!("In file: {}", self.file_path);
    }
}

fn run(config: Configuration) -> Result<(), Box<dyn error::Error>> {
    if config.is_verbose {
        config.print();
        helper::print_divider()
    };

    let haystack = Haystack::build_from_file(config.file_path)?;

    if config.is_verbose {
        haystack.print();
        helper::print_divider()
    };

    let results = search(config.needle, &haystack, &config.options);

    print_search_result(config.needle, &results, config.is_verbose);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Configuration::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    };
}
