use std::fs;
use std::io;

pub const DEFAULT_SEARCH_OPTIONS: SearchOptions = SearchOptions { ignore_case: false };

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
            _ => println!("{}", self.value),
        }
    }
}

pub fn print_search_result<'a>(
    needle: &str,
    results: impl Iterator<Item = MatchResult<'a>>,
    is_verbose: bool,
) -> usize {
    let output_type = if is_verbose {
        ResultOutputType::Verbose
    } else {
        ResultOutputType::Simple
    };

    let mut results = results.peekable();

    if results.peek().is_none() {
        println!("No results for {needle}");
        return 0;
    };

    let mut count = 0;
    for match_result in results {
        match_result.print(&output_type);
        count += 1;
    }
    count
}

pub fn search<'a>(
    needle: &str,
    haystack: &'a Haystack,
    options: &SearchOptions,
) -> impl Iterator<Item = MatchResult<'a>> {
    let ignore_case = options.ignore_case;

    let needle = if ignore_case {
        needle.to_lowercase()
    } else {
        needle.to_string()
    };

    let test = move |line: &str| -> bool {
        if ignore_case {
            line.to_lowercase().contains(&needle)
        } else {
            line.contains(&needle)
        }
    };

    haystack
        .content
        .lines()
        .enumerate()
        .filter(move |(_, line)| test(line))
        .enumerate()
        .map(|(result_index, (line_index, value))| MatchResult {
            value,
            line_number: line_index + 1,
            result_number: result_index + 1,
        })
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

    #[test]
    fn one_result() {
        let needle = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let haystack = build_hs(content);
        let result: Vec<MatchResult> = search(needle, &haystack, &DEFAULT_SEARCH_OPTIONS).collect();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "safe, fast, productive.");
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = build_hs(CASING_TEST_CONTENT);
        let result: Vec<MatchResult> =
            search(needle, &haystack, &SearchOptions { ignore_case: true }).collect();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].value, "Rust:");
        assert_eq!(result[1].value, "Trust me.");
    }

    #[test]
    fn case_sensitive() {
        let needle = "rUsT";
        let haystack = build_hs(CASING_TEST_CONTENT);
        let result: Vec<MatchResult> =
            search(needle, &haystack, &SearchOptions { ignore_case: false }).collect();

        assert_eq!(result.len(), 0);
    }
}
