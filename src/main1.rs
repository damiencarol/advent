use std::io;

fn main1() {
    let lines = io::stdin().lines();
    let mut guess_int_total: u32 = 0;
    let mut array: [u32; 3] = [0; 3];
    for line in lines {
        let guess_int: u32 = match line.unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                for item in IntoIterator::into_iter(array).enumerate() {
                    let (i, x): (usize, u32) = item;
                    if guess_int_total > x {
                        array[i] = guess_int_total;
                        guess_int_total = x;
                    }
                }
                guess_int_total = 0;
                continue;
            }
        };
        // println!("found number: {}", guess_int);
        guess_int_total += guess_int;
        // println!("found total: {}", guess_int_total);
    }
    println!("found max total1: {}", array[0]);
    println!("found max total2: {}", array[1]);
    println!("found max total3: {}", array[2]);
    println!("found max total: {}", array[0]+array[1]+array[2]);
}
