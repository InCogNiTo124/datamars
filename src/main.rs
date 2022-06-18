use clap::{Arg, Command};
use std::io::BufRead;

#[derive(Default)]
struct Mean {
    n: i64,
    result: f64,
}

impl Mean {
    fn apply(&mut self, x: f64) {
        self.n += 1;
        self.result += (x - self.result) / (self.n as f64);
    }

    fn new() -> Mean {
        Mean {n: 0, result: 0.0}
    }
}

fn main() {
    let parser = Command::new("ms")
        .author("Marijan Smetko, msmetko@msmetko.xyz")
        .version("0.0.2")
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .default_value(","),
        )
        .arg(Arg::new("commands").multiple_values(true));
    let matches = parser.get_matches();
    let delimiter = matches.value_of("delimiter").unwrap();
    let vals: Vec<&str> = matches
        .values_of("commands")
        .expect("No commands provided")
        .collect();
    assert!(vals.len() == 2);
    assert!(vals[0].eq("mean"));
    let index = vals[1].parse::<usize>().unwrap() - 1;
    let mut mean = Mean::new();
    let locked_stdin = std::io::stdin();
    for line in locked_stdin.lock().lines() {
        let _line = line.unwrap();
        let parts: Vec<&str> = _line.split(delimiter).collect();
        let number = parts[index].parse::<f64>().unwrap();
        mean.apply(number);
    }
    println!("{}", mean.result);
}
