// B. Calculando divisores

use::std::io::Read;

fn main() {

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let a:i32 = input.next().unwrap().parse().unwrap();
    let mut divisors: Vec<i32> = Vec::new();

    for x in (1..=a).rev() {
        if a % x == 0{
            divisors.push(x);
        }
    }

    for element in divisors {
        println!("{}",element);
    }
}
