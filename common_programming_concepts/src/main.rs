fn main() {
    let x = 5;
    let x = x + 1;
    let age: u8 = 255;

    println!("The value of age is {age}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let myFloat = 2.0; // default float64
    let myMoney: f32 = 4.2252; // float 32

    // addition
    let sumA = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = (-5 / 3); // Results in -1

    // remainder
    let remainder = 43 % 5;

    // boolean
    let flag = true;
    let flag: bool = false;

    // character
    let c = 'z';
    let z: char = 'z'; // using single quotes

    // tuples
    let tup: (i8, u8, f32) = (120, 127, 3.14);
    // let (x, y, z) = tup;
    let x = tup.1;
    println!("The value of x {x}");

    // arrays
    let number = [1,2,3,4,5,6]; // or let number :[i32; 6] = [1,2,3,4,5,6]; here, i32 is the type of each element. The number 6 indicates the array contains six elements.
    let number = [3, 5]; // => a equal to [3,3,3,3,3];
    let first = number[1];
    println!("{first}");


    // functions

    another_function(1);
    let res: i32 = sum(1, 2);
    println!("The value of sum 1 vs 2 is {res}");

    // conditional
    let number = 3;
    if number < 10  {
        println!("Too small!");
    } else {
        println!("Too big!");
    }

    let is_even = if number % 2 == 0 {true} else {false};
    println!("The value of is_even variable is {is_even}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("The value of result is {counter}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number_of_while_loop = 1;
    let mut res_of_while_loop = 0;
    while number_of_while_loop <= 10 {
        res_of_while_loop += number_of_while_loop;
        number_of_while_loop += 1;
    }
    println!("The value of res_of_while_loop {res_of_while_loop}");

    let collections = [10, 20, 30, 40, 50];
    for el in collections {
        println!("Element {el}");
    }

}


fn sum(a: i32, b: i32) -> i32 {
 return a + b;
}

fn another_function (x: i32) {
    println!("The value of x is {x}");
}