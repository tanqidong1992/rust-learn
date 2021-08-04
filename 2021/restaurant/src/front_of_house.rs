pub mod hosting;
mod serve_order{
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){
        super::hosting::add_to_waitlist();
        crate::front_of_house::hosting::add_to_waitlist();
      
    }
}
fn test(){
    self::hosting::seat_at_table();
}