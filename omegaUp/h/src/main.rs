// H. Buscando el mayor

use::std::io::{self,Read};

fn main() {

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.lines();

    let number_of_elements:usize = input.next().unwrap().trim().parse().unwrap();
    let mut elements: Vec<i32> = Vec::new();
    for _ in 0..number_of_elements {
        let num:i32 = input.next().unwrap().trim().parse().unwrap();
        elements.push(num);
    }

    if number_of_elements == 1 {
        println!("{}",elements[0]);
    } else {
        let mut mayor = i32::min_value();
        for element in &elements {
            let actual = *element;
            if actual > mayor {
                mayor = actual;
            }
        }
        println!("{}",mayor);
    }
}
