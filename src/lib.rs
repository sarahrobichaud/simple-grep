use std::fs;
use std::io;

pub struct MatchResult<'a> {
    value: &'a str,
    line_number: usize,
    result_number: usize
}

impl<'a> MatchResult<'a>{
    pub fn print(&self){
        let result_prefix = format!("Result #{}", self.result_number);
        let message = format!("@L{} | {}", self.line_number, self.value);
        println!("{} - {}", result_prefix, message);
    }
}

pub struct Haystack {
    content: String
}

impl Haystack {
    pub fn build_from_file(file_path: &str) -> Result<Haystack, io::Error> {
        let content =  fs::read_to_string(file_path)?;

        Ok(Haystack {
            content
        })
    }

    pub fn print(&self){
        println!("Within text:\n{}", self.content)
    }
}

pub fn search<'a>(needle: &str, haystack: &'a Haystack) -> Vec<MatchResult<'a>> {

    let mut matches : Vec<MatchResult<'a>> = vec![];

    for (i, line) in haystack.content.lines().enumerate() {
        if line.contains(needle) {

            let match_result = MatchResult {
                value: line,
                line_number: i + 1,
                result_number: matches.len() + 1
            };

            matches.push(match_result);
        }
    }

    matches
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let needle = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        let haystack = Haystack {
            content: content.to_string()
        };
        let result = search(needle, &haystack);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "safe, fast, productive.");
    }
}
