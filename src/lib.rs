use std::fs;
use std::error::Error;
use std::process;
use std::env;



pub struct Config { //you must pub also the params of the struct
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("You must enter in this order: query and filename");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // println!("query: {}", config.query);
    // println!("filename: {}", config.filename);

    let content = fs::read_to_string(config.filename)?;
    
    let search_res :Vec<&str>;
   
    
    if config.case_sensitive {
        search_res = search(&config.query, &content).unwrap_or_else(|err| {
            eprintln!("Mmmmh... {} retry with other queries", err);
            process::exit(1);
        });
    }else{
        search_res = search_case_insensitive(&config.query, &content).unwrap_or_else(|err| {
            eprintln!("Mmmmh... {} retry with other queries", err);
            process::exit(1);
        });
    }

    for line in search_res {
        println!("{:?}", line);
    }

    Ok(()) // (()) means we’re calling run for its side effects only
}



// lifetime parameters specify which argument lifetime is connected to the lifetime of the return value
// In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents
// in other words: data returned have the same lifetime of content argument
pub fn search<'a>(query: &str, content: &'a str) -> Result<Vec<&'a str>, &'a str>{ 
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query){
            results.push(line);
        }
    }
    if results.len() >= 1{
        return Ok(results)
    }else{
        Err("No result found for query.")
    }
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Result<Vec<&'a str>, &'a str>{ 
    let mut results = Vec::new();
    let q = query.to_lowercase();
   

    for line in content.lines() {
        if line.to_lowercase().contains(&q){
            results.push(line);
        }
    }
    if results.len() >= 1{
        // println!("{:?}", results);
        return Ok(results)
    }else{
        Err("No result found for query.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "overrated";
        let content = "\
        Super mario is the
most overrated game
In the history of 2d Overrated games";

        assert_eq!(Ok(vec!["most overrated game"]), search(query, content));
    }

    #[test]
    fn one_result_case_insensitive(){
        let query = "overrated";
        let content = "\
        super mario is the
mOst oveRrated gAme
in the history of 2d games";

        assert_eq!(Ok(vec!["mOst oveRrated gAme"]), search_case_insensitive(query, content));
    }
}


