use std::thread;

struct Philosopher {
    name: String,
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
        Philosopher::new("Angelina")
    ];

    for p in &philosophers {
        p.eat();
    }
}
