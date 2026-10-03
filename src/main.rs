use std::{env, fs, io::{Error}};

struct Configuration<'a> {
    query: &'a String,
    file_path: &'a String
}

impl<'a> Configuration<'a> {
    fn new(args: &'a [String]) -> Configuration<'a> {

        // TODO :fix
        let Some(file_path) = args.get(2) else {
            panic!("No file path provided");
        };

        let query = args.get(1).expect("This should always be set if file_path is readable!");

        Configuration {
            query,
            file_path
        }
    }

    fn print(&self) {
        println!("Searching for {}", self.query);
        println!("In file: {}", self.file_path);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Configuration::new(&args);
    config.print();

}



fn read_file_contents(file_path: &String) -> Result<String, Error> {

    let contents_result = fs::read_to_string(file_path);

    match contents_result {
        Ok(file) => Ok(file),
        Err(error) => match error {
            NotFound => panic!("Couldn't find the file")
            _ => Err(error)
        }
    }


}
