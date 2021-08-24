extern crate regex;
extern crate stats;
use std::fs;
use std::fs::{
	DirEntry,
	ReadDir,
};
use regex::Regex;
use stats::{
	MinMax,
	OnlineStats,
};
use std::env; 
fn main() {
	let args:Vec<String>=env::args().collect();
	let dir=args.get(1).expect("No dir path specified!");
    let files:ReadDir=fs::read_dir(dir).expect("read dir fail");
    let mut cis=Vec::new();
    files.for_each(|f|{
	    if f.is_ok(){
		    let entry:DirEntry=f.ok().expect("read dir enrty fail");
            println!("{:?}",entry);
			if let Ok(file_type)=entry.file_type(){
				if file_type.is_file(){
					let content=fs::read_to_string(entry.path()).unwrap();
				    //println!("file:{},content:{}",entry.file_name().to_str().unwrap(),content);
					let mut check_infos=parse_content(&content);
					cis.append(&mut check_infos);
				}
			}
           
	    }
	    
    });
  /*  for ci in cis.iter(){
	    println!("parse result:{:?}",ci);
    }*/
    let mut minMax:MinMax<i32>=cis.iter()
    .filter(|ci|ci.state==ONLINE)
    .map(|ci|ci.use_time)
    .collect();
    println!("online:min:{}.max:{}",minMax.min().unwrap(),minMax.max().unwrap());
    let mut os:OnlineStats =cis.iter()
    .filter(|ci|ci.state==ONLINE)
    .map(|ci|ci.use_time)
    .collect();
     println!("online:avg:{}.variance:{}",os.mean(),os.variance());


  let mut minMax:MinMax<i32>=cis.iter()
    .filter(|ci|ci.state==OFFLINE)
    .map(|ci|ci.use_time)
    .collect();
    println!("offline:min:{}.max:{}",minMax.min().unwrap(),minMax.max().unwrap());
    let mut os:OnlineStats =cis.iter()
    .filter(|ci|ci.state==OFFLINE)
    .map(|ci|ci.use_time)
    .collect();
     println!("offline:avg:{}.variance:{}",os.mean(),os.variance());

}
#[derive(Debug)]
pub struct CheckInfo{
	ip:String,
	state:String,
	use_time:i32,
	log:String,
	
}

impl CheckInfo{
	pub fn new(line:&str,items:&[String])->CheckInfo{
		let ip=items[1].clone();
		let state=items[2].clone();
		let use_time:i32=items[3].parse().expect(line);
		let log=line.to_string().clone();
		CheckInfo{
			ip,
			state,
			use_time,
			log,
		}
	}
}

	const OFFLINE: &'static str = "OFFLINE";
	const ONLINE: &'static str = "ONLINE";
	const OFFLINE_STR: &'static str ="[OFFLINE] finished using time";
	const ONLINE_STR: &'static str = "[ONLINE] finished using time";
	
pub fn parse_content(content:&str)->Vec<CheckInfo>{
	let mut cis=Vec::new();
	let re=Regex::new("\\[[^\\[^\\]]+\\]").unwrap();
	for line in content.lines(){
		 
		if !(line.contains(OFFLINE_STR) || line.contains(ONLINE_STR)){
			continue
		}
		 
		let mut items:Vec<String>=Vec::new();
		for cap in re.captures_iter(line){
			
			let item=cap.at(0).unwrap();
			let item=item[1..item.len()-1].to_string().clone();
			//println!("match:{}",item);
			items.push(item);
		}
		if items.len() == 4{
			let ci=CheckInfo::new(&line,&items);
			cis.push(ci);
			
		}
	}
	return cis;
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_parse(){
		let content="[123],asdjhash[] sadas [32]";
		parse_content(&content);
	}
	
}
