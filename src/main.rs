use clap::{Arg, Command};
use operator::Operator;
use std::io::BufRead;

mod operator;
mod ops;

use ops::geomean::GeoMean;
use ops::harmean::HarMean;
use ops::mean::Mean;
use ops::sum::Sum;
use ops::median::Median;

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
                "median" => Box::new(Median::new()) as Box<dyn Operator>,
                _ => panic!("eror"),
            },
            index,
        }
    }
    pub(crate) fn process(&mut self, parts: &[&str]) {
        self.op
            .apply(parts[self.index].parse().expect("no number provided"));
    }

    fn result(&self) -> f64 {
        self.op.result()
    }
}
fn operator_definitions(operations: Vec<&str>) -> Vec<(&str, &str)> {
    let mut op_definitions: Vec<(&str, &str)> = Vec::new();
    let mut i = 0;
    while i < operations.len() {
        op_definitions.push((operations[i], operations[i + 1]));
        i += 2;
    }
    op_definitions
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
    let operations: Vec<&str> = matches
        .values_of("commands")
        .expect("No commands provided")
        .collect();

    let op_definition = operator_definitions(operations);
    let mut processors: Vec<Processor> = Vec::new();
    for (op_type, arg) in op_definition {
        let index = arg.parse::<usize>().unwrap() - 1;
        processors.push(Processor::new(op_type, index));
    }
    // assert_eq!(vals[0], "mean");
    let locked_stdin = std::io::stdin();
    for line in locked_stdin.lock().lines() {
        let _line = line.unwrap();
        let parts: Vec<&str> = _line.split(delimiter).collect();
        for processor in processors.iter_mut() {
            processor.process(&parts);
        }
    }
    println!("{}", processors.iter().map(|p| p.result().to_string()).collect::<Vec<_>>().join("\t"));
}
