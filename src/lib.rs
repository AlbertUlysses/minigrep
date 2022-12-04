use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not Enough Arguments.");
        }
        let query = &args[1];
        let file_path = &args[2];
        
        Ok(Config {query: query.to_string(),file_path:  file_path.to_string()})
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}
