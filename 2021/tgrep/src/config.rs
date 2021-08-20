#[derive(Debug)]
pub struct Config{
    pub search: String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl  Config{

    pub fn parse_from(args:&Vec<String>) -> Result<Config,String>{
        if args.len() < 4{
            return Result::Err(String::from("Required parameters must be provided!"));
        }
        let config=Config{
            search:args.get(1).unwrap().clone(),
            filename:args.get(2).unwrap().clone(),
            case_sensitive:args.get(3).unwrap().parse().unwrap()
        };
        return Ok(config);
    }
}


