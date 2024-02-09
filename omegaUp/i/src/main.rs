// I. Alcoholic Pilot

use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let input = buffer.lines();
    let mut case_number = 1;
    for line in input {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if numbers[0] == 0 && numbers[1] == 0 && numbers[2] == 0 && numbers[3] == 0 {
            break;
        }
        let c = numbers[0] * numbers[3];
        let o = numbers[1] * numbers[2];
        if c > o {
            println!("Case #{}: You owe me a beer!", case_number);
        } else {
            println!("Case #{}: No beer for the captain.", case_number);
        }
        let num = (numbers[0] * numbers[3]) + (numbers[1] * numbers[2]);
        let den = numbers[0] * numbers[2];
        let p1 = num * 1;
        let p2 = den * 2;
        let gcd = gcd(p1, p2);
        let num2 = p1 / gcd;
        let den2 = p2 / gcd;
        if den2 == 1{
            println!("Avg. arrival time: {}", num2);
        } else {
            println!("Avg. arrival time: {}/{}", num2, den2);
        }
        case_number+=1;
    }
}


fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}