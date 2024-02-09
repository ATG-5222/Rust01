//C. Los Feriapesos

use::std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let precio:i32 = input.next().unwrap().trim().parse().unwrap();
    let feriapesos:i32 = input.next().unwrap().trim().parse().unwrap();

    let pesos = feriapesos/10;
    let pago = precio-pesos;

    println!("${}", if pago < 0 {0} else {pago});
}
