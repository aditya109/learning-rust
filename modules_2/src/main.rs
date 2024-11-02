mod front_of_house;

fn main() {
    // Access the public `add_to_waitlist` function
    front_of_house::hosting::add_to_waitlist();

    // The following line would cause an error because `seat_at_table` is private
    // front_of_house::hosting::seat_at_table();

}
