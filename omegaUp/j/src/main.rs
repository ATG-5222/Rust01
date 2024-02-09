// J. Ordenamiento binario

use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let n: i16 = input.next().unwrap().parse().unwrap();
    let mut numbers: Vec<u128> = Vec::new();
    for _ in 0..n {
        let num: u128 = input.next().unwrap().parse().unwrap();
        numbers.push(num);
    }
    numbers.sort_by_key(|&x| (count_ones(x), x));   
    for &num in &numbers {
        print!("{} ", num);
    }
}

fn count_ones(mut num: u128) -> u128 {
    let mut count = 0;
    while num > 0 {
        count += num % 2;
        num /= 2;
    }
    count
}