fn main() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let a = [1,2,3,4,5];
    let index = 3;
    let element = a[index];
    println!("The value of element is: {}", element);
}
