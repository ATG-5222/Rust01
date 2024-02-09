// L. The Incrementor

use::std::io::Read;

fn main() {

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let number:String = input.next().unwrap().parse().unwrap();
    let result_str = increment_number(&number);

    println!("{}", result_str);
}

fn increment_number(number: &str) -> String {
    let mut result_str = String::new();
    let mut carry = 1;
    for digit in number.chars().rev() {
        if let Some(mut value) = digit.to_digit(10) {
            value += carry;
            carry = value / 10;
            value %= 10;
            result_str.insert(0, char::from_digit(value, 10).unwrap());
        } else {
            break;
        }
    }
    if carry > 0 {
        result_str.insert(0, '1');
    }
    result_str
}