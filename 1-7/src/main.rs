mod front_of_house;

fn main() {
    eat_at_restaurant();
}

pub fn eat_at_restaurant(){
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}