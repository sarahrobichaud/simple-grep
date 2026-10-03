use std::fs;
use std::io;

pub struct SearchOptions {
    ignore_case: bool,
}

impl SearchOptions {
    pub fn build(ignore_case: bool) -> Self {
        Self { ignore_case }
    }
}

pub struct Haystack {
    content: String,
}

impl Haystack {
    pub fn build_from_file(file_path: &str) -> Result<Haystack, io::Error> {
        let content = fs::read_to_string(file_path)?;

        Ok(Haystack { content })
    }

    pub fn print(&self) {
        println!("Within text:\n{}", self.content)
    }
}

pub struct MatchResult<'a> {
    value: &'a str,
    line_number: usize,
    result_number: usize,
}

enum ResultOutputType {
    Simple,
    Verbose,
}

impl<'a> MatchResult<'a> {
    fn print(&self, output_type: &ResultOutputType) {
        match output_type {
            ResultOutputType::Verbose => {
                let result_prefix = format!("Result #{}", self.result_number);
                let message = format!("@L{} | {}", self.line_number, self.value);
                println!("{} - {}", result_prefix, message);
            }
            _ => print!("{}", self.value),
        }
    }
}

pub fn print_search_result(needle: &str, results: &[MatchResult], is_verbose: bool) {
    if results.is_empty() {
        println!("No results for {needle}");
        return;
    }

    let output_type = if is_verbose {
        ResultOutputType::Verbose
    } else {
        ResultOutputType::Simple
    };

    for match_result in results {
        match_result.print(&output_type);
    }
}

pub fn search<'a>(
    needle: &str,
    haystack: &'a Haystack,
    options: &SearchOptions,
) -> Vec<MatchResult<'a>> {
    let mut matches: Vec<MatchResult<'a>> = vec![];

    let normalized_needle = needle.to_lowercase();

    for (i, line) in haystack.content.lines().enumerate() {
        let is_valid_match = if options.ignore_case {
            line.to_lowercase().contains(&normalized_needle)
        } else {
            line.contains(needle)
        };

        if is_valid_match {
            matches.push(MatchResult {
                value: line,
                line_number: i + 1,
                result_number: matches.len() + 1,
            });
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASING_TEST_CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    fn build_hs(content: &str) -> Haystack {
        Haystack {
            content: content.to_string(),
        }
    }

    fn default_search_options() -> SearchOptions {
        SearchOptions { ignore_case: false }
    }

    #[test]
    fn one_result() {
        let needle = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let haystack = build_hs(content);
        let result = search(needle, &haystack, &default_search_options());

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "safe, fast, productive.");
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = build_hs(CASING_TEST_CONTENT);
        let result = search(needle, &haystack, &SearchOptions { ignore_case: true });

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].value, "Rust:");
        assert_eq!(result[1].value, "Trust me.");
    }

    #[test]
    fn case_sensitive() {
        let needle = "rUsT";
        let haystack = build_hs(CASING_TEST_CONTENT);
        let result = search(needle, &haystack, &SearchOptions { ignore_case: false });

        assert_eq!(result.len(), 0);
    }
}
