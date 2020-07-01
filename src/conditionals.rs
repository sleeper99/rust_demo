pub fn run(age:u8, checked_id:bool, person_known:bool) {
    if age >= 21 && checked_id || person_known {
        println!("Bartender: What would you like to drink?");
    } else if age >=21 && !checked_id{
        println!("Bartender: I need to see your ID!");
    } else {
        println!("Bartender: You not old enough to drink!");
    }
}