extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("breed")
    .version("0.1.0")
    .author("Papillon6814")
    .about("clap example cli")
    .arg(Arg::with_name("pa")
        .help("sample positional argument")
        .required(true)
    )
    .arg(Arg::with_name("flg")
        .help("sample flag")
        .short("f")
        .long("flag")
    )
    .arg(Arg::with_name("opt")
        .help("sample option")
        .short("o")
        .long("opt")
        .takes_value(true)
    )
    .subcommand(SubCommand::with_name("sub")
        .about("sample subcommand")
        .arg(Arg::with_name("subflg")
            .help("sample flag by sub")
            .short("f")
            .long("flag")
        )
    );

    // parse arguments
    let matches = app.get_matches();

    if let Some(o) = matches.value_of("pa") {
        println!("Value for pa: {}", o);
    }

    if let Some(o) = matches.value_of("opt") {
        println!("Value for opt: {}", o);
    }

    // on/off flag
    println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

    // obtain the result of sub subcommand
    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub");
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}
