use std::fs;

pub fn read_file(path: &String) -> String{
    let file_data = fs::read_to_string(path).expect("File does not exist.");
    return file_data;
}

pub fn serach<'a>(content: &'a String, query: &'a String) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[derive(Debug)]
pub struct Config{
    pub query: String,
    pub path: String,
    pub case_insensetive: bool
}

impl Config {
    pub fn new(params: &[String]) -> Result<Config, &'static str>{
        if params.len() < 3 {
           return Err("Usage: Grep [-i] text path"); 
        }
        if params.len() == 3{
             return Ok(Config{
                query: params[1].clone(),
                path: params[2].clone(),
                case_insensetive: false
            });
        }
        else if params.len() == 4 && params[1] == "i"{
            return Ok(Config{
                query: params[2].clone(),
                path: params[3].clone(),
                case_insensetive: true
            });
        }
        Err("Usage: Grep [-i] text path")
    }
}
