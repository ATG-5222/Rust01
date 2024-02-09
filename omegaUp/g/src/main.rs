// G.Perímetro de asteriscos

use::std::io::Read;

fn main() {

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let h:i32 = input.next().unwrap().parse().unwrap();
    let w:i32 = input.next().unwrap().parse().unwrap();

    for i in 1..=h {
        if i == 1 || i == h {
            for _ in 1..=w {
                print!("* ");
            }
        } else {
            print!("* ");
            if w > 1 {
                for _ in 2..w {
                    print!(" ");
                }
                print!("*");
            }
        }
        println!();
    }

}
