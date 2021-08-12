
use rand::prelude::*;

fn main() {
    
    let secret=rand::thread_rng().gen_range(1..=100);
    println!("secret number:{}",secret);

    loop{
        let mut guess=String::new();
        std::io::stdin().read_line(& mut guess).unwrap();
        println!("input:{}",guess);
        let guess:i32=guess.trim().parse().unwrap();
        if guess == secret {
            println!("You Win!");
            break;
        }else if secret <= guess{
            println!("Too big!")
        }else{
            println!("Too small!")
        }

    }
}
