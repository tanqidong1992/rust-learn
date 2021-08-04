fn main() {
    let a=1;
    let b=a;
    println!("a:{},b:{}",a,b);
    let s=String::from("Hello World!");
    let mut s1=s;
    //error[E0382]: borrow of moved value: `s`
    //println!("{}",s);
    let index=first_word_index(&s1);
    println!("first word:{}",&s1[..index]);
    let s11=&s1[..];
    //error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
    //s1.clear();
    println!("{}",s11)
}

fn first_word_index(s: & String)->usize {
    
    for (i,&c) in s.as_bytes().iter().enumerate(){
        
        if c == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word(s: &String) -> &str{

    let bytes=s.as_bytes();
    for (i,&c) in bytes.iter().enumerate(){

        if c==b' ' {
            return &s[0..i];
        }
    }
    &s[..]

}
