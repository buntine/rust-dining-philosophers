use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
      Philosopher {
          name: name.to_string(),
      }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Bunts"),
        Philosopher::new("Mel"),
        Philosopher::new("Grace"),
        Philosopher::new("Rohan"),
        Philosopher::new("Angelina"),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();


    for h in handles {
        h.join().unwrap();
    }
}
