use std::{env, error, fs, io, process};

struct Configuration<'a> {
    query: &'a String,
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
            query,
            file_path
        })
    }

    fn print(&self) {
        println!("Searching for {}", self.query);
        println!("In file: {}", self.file_path);
    }
}

struct Haystack {
    content: String,
}

impl Haystack {
    fn print(&self){
        println!("Within text:\n{}", self.content)
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

    println!("-----------------");

    let haystack = get_haystack(config.file_path)?;
    haystack.print();

    Ok(())
}


fn get_haystack(file_path: &String) -> Result<Haystack, io::Error> {
    let content = fs::read_to_string(file_path)?;
    Ok(Haystack { content })
}
