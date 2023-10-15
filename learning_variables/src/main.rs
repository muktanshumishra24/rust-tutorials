use core::num;
use std::result;

fn main() {
    let x =5;
    println!("The Value of x is {}", x);
    //Shadowing Means you can use same name for two vars
    let x="Six";
    println!("The Value of x is {}", x);

    //Const Used To Ensure Immutability.
    const BIRTHYEAR:u32 = 2002;
    println!("The Const is {}", BIRTHYEAR);

    //DATATYPES
    let sum = another_function(11,22);
    println!("The Sum of these two is {}", sum);

    if sum < 10{
        println!("Small Sum Fr :/");
    } else if sum==33 {
        println!("Goddamn :=|")
    } else{
        println!("STFU :X")
    }

    let mut counter =0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter;
        }
    };

    println!("The Result is {}", result);

    let mut number = 3;

    while number !=0{
        println!("The Number {}", number);

        number -= 1;
    }

    println!("Off the While Loop");

    let a = [1,2,3,4,5];

    for element in a.iter(){
        println!("The Element Is {}", element);
    }
}

fn another_function(x: i32 , y: i32) -> i32 {
    println!("The Value of x is {}", x);
    println!("The Value of y is {}", y);
    x+y
}
