fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;

    let y = y + 1; // shadowing!

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of x.0 is: {}", five_hundred);

    let a = [1, 2, 3, 4, 5];
    println!("The value of the third element of the array a is: {}", a[2]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "The value of the second element of the array a is: {}",
        a[1]
    );

    let a = [3; 5];
    println!("The value of the first element of the array a is: {}", a[0]);
}
