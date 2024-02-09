// K. Desarrollos Binomiales

use::std::io::Read;

fn main() {

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let n:i128 = input.next().unwrap().trim().parse().unwrap();

    if n == 0 {
        print!("1");
    } else {
        for i in 0..=n {
            let coefficient = binomial_coefficient(n, i);
            if i > 0 {
                print!("+");
            }
            if coefficient != 1 {
                print!("{}", coefficient);
            }
            if n - i > 0 {
                print!("x");
                if n - i > 1 {
                    print!("^{}", n - i);
                }
            }
            if i > 0 {
                print!("y");
                if i > 1 {
                    print!("^{}", i);
                }
            }
        }
        println!();
    }
}

fn binomial_coefficient(n: i128, k: i128) -> i128 {
    if k == 0 || k == n {
        return 1;
    }
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}