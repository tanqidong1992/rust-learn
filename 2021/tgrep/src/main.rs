mod config;

use config::*;
fn main() {
    let args:Vec<String>=std::env::args().collect();
    //println!("{:?}",args);

    let (keyword,filepath)=parse_config(&args);
    let content=std::fs::read_to_string(filepath).unwrap();
    //println!("read file:{}\n{}\n",filepath,content);
    let lines:Vec<&str>=content.split('\n').collect();
    //println!("line size:{}",lines.len());
    for line in lines{
        if line.contains(keyword) {
            println!("{}",line);
        }
    }
}

fn parse_config(args:& Vec<String>) -> (&str,&str){
    if args.len() < 4 {
        panic!("Required parameters must provided!")
    }
    (&args[1],&args[2])
}
