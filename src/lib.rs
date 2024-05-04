use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // NOTE: Use string to return errors since we're not using
    // predefined error types
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// NOTE: Use `Box<dyn Error>` to return dynamic error types
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    // NOTE: Propagate errors with `?`
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n {}", contents);

    Ok(())
}
