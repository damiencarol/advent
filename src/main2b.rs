use std::io;

///
/// A for Rock (1), B for Paper (2), and C for Scissors (3).
/// in response: X for Rock, Y for Paper, and Z for Scissors.
///
fn rps(opponent: String, mine: String) -> u32 {
    let mut total: u32 = 0;
    if mine == "X" && opponent == "A" { //  Loose
        total += 3;
    }
    if mine == "X" && opponent == "B" { //  Loose
        total += 1;
    }
    if mine == "X" && opponent == "C" { //  Loose
        total += 2;
    }
    if mine == "Y" && opponent == "A" { //  Draw
        total += 1;
    }
    if mine == "Y" && opponent == "B" { //  Draw
        total += 2;
    }
    if mine == "Y" && opponent == "C" { //  Draw
        total += 3;
    }
    if mine == "Z" && opponent == "A" { //  Win
        total += 2;
    }
    if mine == "Z" && opponent == "B" { //  Win
        total += 3;
    }
    if mine == "Z" && opponent == "C" { //  Win
        total += 1;
    }

    if mine == "X" {
        total += 0; // Loose
    }
    if mine == "Y" {
        total += 3; // Draw
    }
    if mine == "Z" {
        total += 6; // Win
    }

    return total;
}

///
/// A for Rock, B for Paper, and C for Scissors.
/// in response: X for Rock, Y for Paper, and Z
///
fn main2b() {
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
