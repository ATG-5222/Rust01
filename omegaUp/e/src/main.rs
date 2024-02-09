// E. Número y Pico

use::std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let n: u32 = input.next().unwrap().parse().unwrap();
    let resultado = numero_y_pico(n);
    println!("{}", resultado);
}

fn numero_y_pico(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    let mitad = n / 2;
    n + numero_y_pico(mitad)
}