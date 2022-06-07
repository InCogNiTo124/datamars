use clap::{Arg, Command};

fn main() {
    let parser = Command::new("ms")
        .author("Marijan Smetko, msmetko@msmetko.xyz")
        .version("0.0.1")
        .arg(
            Arg::new("delimiter")
            .short('d')
            .long("delimiter")
            .default_value(",")
        ).arg(
            Arg::new("commands")
            .multiple_values(true)
        );
    let matches = parser.get_matches();
    println!("{:?}", matches.value_of("delimiter"));
    let vals: Vec<&str> = matches.values_of("commands").unwrap().collect();
    println!("{:?}", vals);
}
