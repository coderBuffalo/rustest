extern crate rand;
use rand::Rng;

use std::thread;
use std::sync::{Arc, Mutex};

struct Table {
    forks: Vec<Mutex<()>>
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        let rand_ms = rand::thread_rng().gen_range(1000, 3000);

        println!("{} 正在吃饭中... ...", self.name);

        thread::sleep_ms(rand_ms);

        println!("\n\n===================");
        println!("{} 吃完了!!!", self.name);
        println!("===================\n\n");
    }
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("关羽", 0, 1),
        Philosopher::new("张飞", 1, 2),
        Philosopher::new("黄忠", 2, 3),
        Philosopher::new("马超", 3, 4),
        Philosopher::new("赵云", 0, 4),
    ];

    let handlers: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handlers {
        h.join().unwrap();
    }
}