extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("请猜测数字,1-100之间！");

    let mut count = 0;
    let rand_num = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut number= String::new();
        io::stdin().read_line(&mut number)
            .ok().expect("获取输入失败！");

        let number: u32 =  match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("请输入数字!\n");continue;},
        };

        match number.cmp(&rand_num) {
            Ordering::Less => { println!("\n{} 太小！", number); count += 1;},
            Ordering::Greater => { println!("\n{} 太大！", number); count += 1;},
            Ordering::Equal => { println!("{} 你赢了!", number); break;},
        }

    }

    println!("你一共猜测了 {} 次", count);

}
