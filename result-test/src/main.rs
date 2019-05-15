/** 
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;
*/
use std::{
    fs,
    fs::File,
    io,
    io::{
        ErrorKind,
        Read,
    },
};
fn main() {
    /** 
     let f=File::open("hello.txt");
   let f= match f {
       Ok(file)=>file,
       Err(error)=>{
           panic!("There was a problem opening the file the error:{:?}",error);
       },
   };
    */
   

/** 
   let f=File::open("hello.txt");
   let f= match f {
       Ok(file)=>file,
       Err(error)=>match error.kind(){
           ErrorKind::NotFound=>match File::create("hello.txt"){
              Ok(fc)=>fc,
              Err(e)=> panic!("try tpo create file but there is a problem error:{:?}",e), 
           },
           other_error => panic!("There was a problem opening the file the error:{:?}",other_error),
       },
   };
   */
  let f=File::open("hello.txt")
    .map_err(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("try to create file but there is a problem error:{:?}",error);
            })
        }else{
            panic!("there war a problem opening the file: {:?}",error);
        }
    });
}

fn read_username_from_file()->Result<String,io::Error>{
    let f=File::open("hello.txt");
    let mut f=match f{
        Ok(file)=>file,
        Err(e)=> return Err(e),
    };
    let mut s=String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }

}
fn read_username_from_file1()->Result<String,io::Error>{
    let mut f=File::open("hello.txt")?;
    let mut s=String::new();
    f.read_to_string((&mut s))?;
    Ok(s)
}
fn read_username_from_file2()->Result<String,io::Error>{
    let mut s=String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn read_username_from_file3()->Result<String,io::Error>{
    fs::read_to_string("hello.txt")
}
