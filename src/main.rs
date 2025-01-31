use std::io;

fn main() {
    elementary1();
    let _ = elementary2();
    let _ = elementary3();
    let _ = elementary4();
}

fn elementary1() {
    println!("Elementary 1:");
    println!("Hello, World");
}

fn elementary2() -> io::Result<()> {
    println!("Elementary 2:");

    let mut input = String::new();

    println!("What is your name?");
    
    io::stdin().read_line(&mut input)?;

    println!("Hello, {}", input.trim());

    Ok(())
}

fn elementary3() -> io::Result<()> {
    println!("Elementary 3:");

    let mut input = String::new();

    println!("What is your name?");
    
    io::stdin().read_line(&mut input)?;

    if input.trim() == "Alice" || input.trim() == "Bob" {
        println!("Hello, {}", input.trim());
    } else {
        println!("You aren't Alice or Bob!");
    };

    Ok(())
}

fn elementary4() -> io::Result<()> {
    println!("Elementary 4:");

    let mut input = String::new();

    println!("Give me a number");
    
    io::stdin().read_line(&mut input)?;

    let _num = input.parse::<u32>();
    let i: u32 = 0;

    while i < _num {
        print!("{} ", i);
    };


    Ok(())
}
