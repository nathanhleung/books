use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("{} + 1 = {}", num, add_one::add_one(num));
    println!("{} + 2 = {}", num, add_two::add_two(num));
}
