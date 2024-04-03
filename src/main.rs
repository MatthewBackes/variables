fn main() {
    // Test program for variables and comments
    // Another test line for comment blocks
    //
    // Once more
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let emoji = '👊';
    
    println!("BEHOLD! THOUSAND FIST ATTACK! {emoji}{emoji}{emoji}{emoji}{emoji}{emoji}{emoji}"); // One more test line

    let tup = (50, '🔊', 6.7, "MAR");
    println!("Printing tuple: {:?} {:?} {:?} {:?} ", tup.0, tup.1, tup.2, tup.3);
}
