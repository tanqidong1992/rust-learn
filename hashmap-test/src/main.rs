use std::collections::HashMap;
fn main() {
    //println!("Hello, world!");
   let mut scores=HashMap::new();
   scores.insert(String::from("Blue"),10);
   scores.insert(String::from("Yello"), 20);
   let teams=vec![String::from("Blue"),
   String::from("Yellow")];
   let initial_scores=vec![10,50];
   let scores:HashMap<_,_>=teams.iter()
       .zip(initial_scores.iter())
       .collect();

   let team_name=String::from("Blue");
   let score=scores.get(&team_name);
   match score{
       Some(s)=>println!("the team:{} score is:{}",team_name,s),
       None=>println!("the team:{} score is: None",team_name),
   }

   for (k,v) in &scores{
       println!("{}:{}",k,v);
   }
   
   let text="hello world wonderful world";
   let mut map=HashMap::new();
   for word in text.split_whitespace(){
       let count=map.entry(word).or_insert(0);
       *count+=1;
   }
   for (k,v) in &map{
       println!("{}:{}",k,v);
   }
   println!("map:{:?}",map);

}
