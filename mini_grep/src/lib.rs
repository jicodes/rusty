use std::fs;
use std::error::Error;
use std::env;

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

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
      args.next(); // skip the program name

      let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
      };

      let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a file name"),
      };

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Config {query, filename, case_sensitive})
  }
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//   let mut results = Vec::new();
//   for line in contents.lines() {
//     if line.contains(query) {
//       results.push(line);
//     }
//   }
//   results
// }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_ne!(
      vec!["safe, fast, productive.", "Duct tape."],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
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