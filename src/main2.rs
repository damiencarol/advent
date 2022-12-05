use std::io;

///
/// A for Rock, B for Paper, and C for Scissors.
/// in response: X for Rock, Y for Paper, and Z
///
fn rps(opponent: String, mine: String) -> u32 {
    let mut total: u32 = 0;
    if mine == "X" {
        total += 1;
    }
    if mine == "Y" {
        total += 2;
    }
    if mine == "Z" {
        total += 3;
    }

    if mine == "X" && opponent == "A" {
        total += 3; // Draw
    }
    if mine == "X" && opponent == "B" {
        total += 0; // Loose
    }
    if mine == "X" && opponent == "C" {
        total += 6; // Win
    }

    if mine == "Y" && opponent == "A" {
        total += 6; // Draw
    }
    if mine == "Y" && opponent == "B" {
        total += 3; // Loose
    }
    if mine == "Y" && opponent == "C" {
        total += 0; // Win
    }

    if mine == "Z" && opponent == "A" {
        total += 0; // Draw
    }
    if mine == "Z" && opponent == "B" {
        total += 6; // Loose
    }
    if mine == "Z" && opponent == "C" {
        total += 3; // Win
    }
    return total;
}

///
/// A for Rock, B for Paper, and C for Scissors.
/// in response: X for Rock, Y for Paper, and Z
///
fn main2() {
    let lines = io::stdin().lines();
    let mut total: u32 = 0;
    for line in lines {
        match line {
            Ok(str) => {
                let split : Vec<String> = str.split(" ").map(|s| s.to_string()).collect();

                println!("opponent: {:?}", split[0]);
                println!("mine: {:?}", split[1]);
                total += rps(split[0].clone(), split[1].clone());
            }
            Err(_) => println!("error"),
        };
    }
    println!("total: {}", total);
}
