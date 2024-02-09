// D. Imprimiendo enteros por paridad

use::std::io::Read;

fn main() {

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let number_of_elements:i8 = input.next().unwrap().parse().unwrap();
    let mut elements: Vec<i8> = Vec::new();
    for _ in 0..number_of_elements {
        let num:i8 = input.next().unwrap().parse().unwrap();
        elements.push(num);
    }
    let condition:i8 = input.next().unwrap().parse().unwrap();

    if condition == 0 {
        for element in elements {
            if element%2 == 0 {
                print!("{} ", element);
            } 
        }
    } else {
        for element in elements {
            if element%2 != 0 {
                print!("{} ", element);
            } 
        }
    }
}
