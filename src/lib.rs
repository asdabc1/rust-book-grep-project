use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::env;

fn read_from_file(filename: &String) -> Result<String, Error> {
    let mut f = File::open(filename)?;

    let mut val = String::new();
    f.read_to_string(&mut val)?;

    Ok(val)
}

struct PassedArguments<'a> {
    file_name: &'a String,
    query: &'a String,
    case_sensitive: bool
}

impl<'a> PassedArguments<'a> {
    pub fn new(args: &[String]) -> Result<PassedArguments, Error> {
        if args.len() < 3 {
            return Err(std::io::Error::other("Too little arguments!"));
        }

        let case = env::var("CASE_INSENSITIVE").is_err();

        Ok(PassedArguments{file_name: &args[2], query: &args[1], case_sensitive: case})
    }
}

pub fn run(args: &[String]) ->Result<(), Box<Error>> {
    let args = PassedArguments::new(args)?;

    let file_contents = read_from_file(&args.file_name)?;

    if args.case_sensitive {
        for line in search(args.query, file_contents.as_str()) {
            println!("{line}");
        }
    }
    else {
        for line in search_case_insensitive(args.query, file_contents.as_str()) {
            println!("{line}");
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn read_from_file_works() {
        let file = String::from("poem.txt");
        let val = read_from_file(&file).unwrap();
        assert!(val.contains("I'm nobody! Who are you?"));
    }

    #[test]
    #[should_panic]
    fn passed_args_new_wont_unwrap_with_few_args() {
        let test_vec = vec!["raz".to_string()];

        let p = PassedArguments::new(&test_vec).unwrap();
    }

    #[test]
    fn passed_args_new_works() {
        let test_vec = vec!["placeholder".to_string(), "query".to_string(), "filename".to_string()];

        let args = PassedArguments::new(&test_vec).unwrap();
        assert_eq!(args.query, "query");
        assert_eq!(args.file_name, "filename");
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}