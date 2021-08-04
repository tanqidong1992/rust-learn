#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_eat_at_restaurant(){
        super::eat_at_restaurant();
    }
}

mod front_of_house;

use front_of_house::hosting;
use std::io::Result as IoResult;
use std::result::Result;
use std::{
    cmp,
    io
};
use std::collections::*;
pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
