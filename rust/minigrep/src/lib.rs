use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    // Skip first argument (name of program)
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Please provide a query string"),
    };
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Please provide a filename"),
    };

    let mut case_insensitive = false;
    // Allow case insensitivity to be set by a command-line argument
    if let Some(arg) = args.next() {
      case_insensitive = arg == "--case-insensitive";
    }
    // If command-line argument isn't provided or doesn't match, also check
    // the environment
    if !case_insensitive {
      case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
    }

    Ok(Config {
      query,
      filename,
      case_sensitive: !case_insensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}