use std::collections::HashMap;

fn main() {
    println!("Closure Test!");
    let mut cacher=Cacher::new(|x|->u32{x});
    assert_eq!(cacher.get(1),1);
    assert_eq!(cacher.get(2),2);
    assert_eq!(cacher.get(3),3);
}

struct Cacher<T> where T:Fn(u32)->u32{
    caculator:T,
    cached_values:HashMap<u32,u32>,

} 

impl <T> Cacher<T> where T:Fn(u32)->u32{
    
    fn new(caculator:T)->Cacher<T>{
        Cacher{
            caculator:caculator,
            cached_values:HashMap::new()
        }
    }

    fn get(&mut self,args:u32)->u32{
        if self.cached_values.contains_key(&args) {
            *self.cached_values.get(&args).unwrap()
        }else{
            let value=(self.caculator)(args);
            self.cached_values.insert(args, value);
            value
        }
    }
}

