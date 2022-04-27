fn main() {
    println!("Hello, world!");

    another_function();

    add_numbers(5, 4);

    //expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function.");
}

fn add_numbers(x: i32, y: i32) {
    println!("The result of adding {} and {} is {}", x, y, x + y);
}
