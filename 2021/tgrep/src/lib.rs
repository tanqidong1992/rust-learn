mod config;

#[cfg(test)]
mod tests{

    #[test]
    fn test_parse_config(){

        use crate::config::*;

        let args=vec![String::from("a"),String::from("a"),String::from("b"),String::from("true")];
        let config=Config::parse_from(&args).unwrap();
        println!("{:?}",config);
        assert_eq!(config.filename,"b");
        assert_eq!(config.search,"a");
        assert!(config.case_sensitive);
    }
}