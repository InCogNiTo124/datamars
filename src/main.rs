extern crate core;

use clap::{Arg, ArgAction, Command};
use csv;
use ops::Operator;

mod ops;

use ops::GeoMean;
use ops::HarMean;
use ops::Max;
use ops::Mean;
use ops::Median;
use ops::Min;
use ops::Std;
use ops::Sum;

struct Processor {
    op: Box<dyn Operator>,
    index: usize,
}

impl Processor {
    fn new(op_type: &str, index: usize) -> Self {
        Self {
            op: match op_type {
                "mean" => Box::new(Mean::new()) as Box<dyn Operator>,
                "min" => Box::new(Min::new()) as Box<dyn Operator>,
                "max" => Box::new(Max::new()) as Box<dyn Operator>,
                "sum" => Box::new(Sum::new()) as Box<dyn Operator>,
                "geomean" => Box::new(GeoMean::new()) as Box<dyn Operator>,
                "harmean" => Box::new(HarMean::new()) as Box<dyn Operator>,
                "median" => Box::new(Median::new()) as Box<dyn Operator>,
                "std" => Box::new(Std::new()) as Box<dyn Operator>,
                _ => panic!("eror"),
            },
            index,
        }
    }
    pub(crate) fn process(&mut self, parts: &[String]) {
        self.op
            .apply(parts[self.index].parse().expect("no number provided"));
    }

    fn result(&self) -> f64 {
        self.op.result()
    }
}
fn operator_definitions<'a>(operations: &'a [&str]) -> Vec<(&'a str, usize)> {
    // fn operator_definitions<'a>(operations: &'a[&str]) -> &[(&str, &str)] {
    let mut op_definitions: Vec<(&str, usize)> = Vec::new();
    let mut i = 0;
    while i < operations.len() {
        let x = operations[i + 1].parse::<usize>().unwrap() - 1;
        op_definitions.push((operations[i], x));
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
        .arg(
            Arg::new("headers-in")
                .long("headers-in")
                .action(ArgAction::SetTrue)
                .takes_value(false)
                .default_value_if("headers", Some("true"), Some("true")),
        )
        .arg(
            Arg::new("headers-out")
                .long("headers-out")
                .action(ArgAction::SetTrue)
                .takes_value(false)
                .default_value_if("headers", Some("true"), Some("true")),
        )
        .arg(Arg::new("headers").short('H').action(ArgAction::SetTrue))
        .arg(Arg::new("commands").multiple_values(true));
    let matches = parser.get_matches();
    let delimiter = matches.value_of("delimiter").unwrap();
    assert_eq!(delimiter.len(), 1);
    let headers_in = *matches.get_one::<bool>("headers-in").unwrap_or(&false);
    let headers_out = *matches.get_one::<bool>("headers-out").unwrap_or(&false);
    let operations: Vec<&str> = matches
        .values_of("commands")
        .expect("No commands provided")
        .collect();

    let op_definition = operator_definitions(&operations);
    let mut processors: Vec<Processor> = Vec::new();
    for (op_type, arg) in op_definition.as_slice() {
        processors.push(Processor::new(*op_type, *arg));
    }

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .has_headers(headers_in)
        .from_reader(std::io::stdin());

    for result in csv_reader.records() {
        let record = result.unwrap();
        let parts = record.iter().map(String::from).collect::<Vec<String>>();
        for processor in &mut processors {
            processor.process(&parts);
        }
    }

    let headers = csv_reader.headers().unwrap();
    if headers_out {
        if headers_in {
            println!(
                "{}",
                op_definition
                    .iter()
                    .map(|t| format!("{}({})", t.0, headers.get(t.1).unwrap()))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        } else {
            println!(
                "{}",
                op_definition
                    .iter()
                    .map(|t| format!("{}({})", t.0, t.1 + 1))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }

    println!(
        "{}",
        processors
            .iter()
            .map(|p| p.result().to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
