use std::io;

fn main() {
    loop {
    println!("Enter a choice: ");
    println!("1) Add\n2) Subtract\n3) Multiply\n4) Divide\n5) Exit\n");

    
    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = match number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the first number: ");
    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a: u32 = match a.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("Enter the second number: ");
    let mut b = String::new();

    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    let b: u32 = match b.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };



    match number{
        1=>{
            add(a, b);
        },
        2=>{
            subtract(a, b);
        },
        3=>{
            multiply(a, b);
        },
        4=>{
            divide(a, b);
        },
        5=>{
            println!("Exit");
            break;
        },
        _=>println!("Enter a valid key"),
       }



    
}
    
}

fn add(a: u32, b: u32){
    let result = a + b;
    println!("The result is: {}", result);
}

fn subtract(a: u32, b: u32){
    let result = a - b;
    println!("The result is: {}", result);
}

fn multiply(a: u32, b: u32){
    let result = a * b;
    println!("The result is: {}", result);
}

fn divide(a: u32, b: u32){
    let result = a / b;
    println!("The result is: {}", result);
}
