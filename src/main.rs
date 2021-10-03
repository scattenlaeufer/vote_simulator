use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

fn main() {
    fn usize_validator(s: String) -> Result<(), String> {
        match s.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Input must be an unsigned integer!".to_string()),
        }
    }

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("voters")
                .short("v")
                .long("voters")
                .help("Number of voters")
                .takes_value(true)
                .default_value("10")
                .validator(usize_validator)
                .multiple(false),
        )
        .arg(
            Arg::with_name("positions")
                .short("p")
                .long("positions")
                .help("Number of postitons to be elected per constituency")
                .takes_value(true)
                .default_value("1")
                .validator(usize_validator)
                .multiple(false),
        )
        .arg(
            Arg::with_name("constituencies")
                .short("c")
                .long("constituencies")
                .help("Number of constituencies")
                .takes_value(true)
                .default_value("1")
                .validator(usize_validator)
                .multiple(false),
        )
        .get_matches();

    vote_simulator::run(
        matches
            .value_of("voters")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        matches
            .value_of("positions")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        matches
            .value_of("constituencies")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );
}
