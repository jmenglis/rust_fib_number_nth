use std::io;

fn main() {
    let mut nth = String::new();
    let mut prev_num: isize = 0;
    let mut cur_num: isize = 1;
    let mut count: i32 = 1;
    println!("Please enter the number in the Fib Sequence");
    match io::stdin().read_line(&mut nth) {
        Ok(_n) => {
            let nth: i32 = nth.trim().parse().expect("Please enter a number");
            if nth == 0 {
                return println!("0");
            }
            while count < nth {
                let hold: isize = cur_num;
                cur_num = prev_num + cur_num;
                prev_num = hold;

                count = count + 1;
            }
            println!("{}", cur_num);
         }
        Err(error) => println!("Error: {}", error),
    }
}
