fn main() {
    // Ownership

    // 1 Variable = 1 Owner (Only 1 is Possible)
    // Owner Goes outta scope the variable value will be dropped

    { // s is not valid here.    
        let s = String:: from("hello");  // s is valid from this point forward
        // utilise s as a variable.
    } // this scope is now over, and s is no longer valid.

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("Hello");
    let s2 = s1; //Shallow Copy

    println!("{}", s1); //error

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    //Works Now

    //Ownership 
    let s = String::from("Hello!");
    takes_ownership(s);
    println!("{}",s); //error

    let x = 5;
    makes_copy(x);
    println!("{}",x); // good to go
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}