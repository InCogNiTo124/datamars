use clap::{Arg, Command};
use operator::operator::Operator;
use std::io::BufRead;

mod operator;
mod ops;

use ops::mean::Mean;
use ops::sum::Sum;
use ops::geomean::GeoMean;
use ops::harmean::HarMean;

struct Processor {
    op: Box<dyn Operator>,
    index: usize,
}

impl Processor {
    fn new(op_type: &str, index: usize) -> Self {
        Processor {
            op: match op_type {
                "mean" => Box::new(Mean::new()) as Box<dyn Operator>,
                "sum" => Box::new(Sum::new()) as Box<dyn Operator>,
                "geomean" => Box::new(GeoMean::new()) as Box<dyn Operator>,
                "harmean" => Box::new(HarMean::new()) as Box<dyn Operator>,
                _ => panic!("eror"),
            },
            index,
        }
    }
    pub(crate) fn process(&mut self, parts: Vec<&str>) {
        self.op
            .apply(parts[self.index].parse().expect("no number provided"));
    }

    fn result(&self) -> f64 {
        self.op.result()
    }
}

fn main() {
    let parser = Command::new("ms")
        .author("Marijan Smetko, msmetko@msmetko.xyz")
        .version("0.2.0")
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
    assert_eq!(vals.len(), 2);
    // assert_eq!(vals[0], "mean");
    let index = vals[1].parse::<usize>().unwrap() - 1;
    let mut processor = Processor::new(vals[0], index);

    let locked_stdin = std::io::stdin();
    for line in locked_stdin.lock().lines() {
        let _line = line.unwrap();
        let parts: Vec<&str> = _line.split(delimiter).collect();
        processor.process(parts);
    }
    println!("{}", processor.result());
}
