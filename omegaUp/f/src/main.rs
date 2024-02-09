// F. Bit de paridad

use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let number:String = input.next().unwrap().parse().unwrap();

    let count_ones = number.chars().filter(|&c| c == '1').count();
    let parity_bit = if count_ones % 2 == 0 { '0' } else { '1' };

    let output_str = format!("{}{}", number, parity_bit);
    println!("{}", output_str);
}
