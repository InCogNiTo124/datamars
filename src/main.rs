use clap::{Arg, Command};
use std::io::BufRead;

fn main() {
    let parser = Command::new("ms")
        .author("Marijan Smetko, msmetko@msmetko.xyz")
        .version("0.0.1")
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .default_value(","),
        )
        .arg(Arg::new("commands").multiple_values(true));
    let matches = parser.get_matches();
    let delimiter = matches.value_of("delimiter").unwrap();
    println!("{:?}", delimiter);
    let vals: Vec<&str> = matches.values_of("commands").unwrap().collect();
    assert!(vals.len() == 2);
    assert!(vals[0].eq("mean"));
    assert!(vals[1].eq("1"));
    println!("{:?}", vals);
    let locked_stdin = std::io::stdin().lock();
    for line in locked_stdin.lines() {
        let _line = line.unwrap();
        let parts: Vec<&str> = _line.split(delimiter).collect();
        println!("{:?}", parts);
    }
}
