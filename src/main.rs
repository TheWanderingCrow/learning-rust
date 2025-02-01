use std::io;

fn main() {
    let _ = elementary6();
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

    let x: u32 = input.trim().parse().expect("Input not an Integer");
    let mut i: u32 = 0;

    while i < x + 1 {
        let s = i.to_string();
        i += 1;
        print!("{} ", s);
    }

    Ok(())
}

fn elementary5() -> io::Result<()> {
    println!("Elementary 5:");

    let mut input = String::new();

    println!("Give me a number");

    io::stdin().read_line(&mut input)?;

    let x: u32 = input.trim().parse().expect("Input not an Integer");
    let mut i: u32 = 0;

    while i < x + 1 {
        let s = i.to_string();
        if i % 3 == 0 || i % 5 == 0 {
            print!("{} ", s);
        };
        i += 1;
    }

    Ok(())
}

fn elementary6() -> io::Result<()> {
    println!("Elementary 6:");

    let mut input = String::new();

    println!("Give me a number");

    io::stdin().read_line(&mut input)?;
    let num: u32 = input.trim().parse().expect("Input not an Integer");

    Ok(())
}

fn elem6_helper1(num: u32) {}
