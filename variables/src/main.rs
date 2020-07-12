fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = 5;
    
    // shadowing
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The len of spaces is: {}", spaces);
}
