use clap::{App, Arg};

fn main() {
    // {:?} instead of {} for the placeholder. This is
    // an instruction to print a Debug version of the structure,
    // which will format the output in a debugging context.
    //println!("Arguments DBG: {:?}", std::env::args());

    let matches = App::new("echor")
        .version("0.1.0")
        .author("JS <orbuluh@gmail.com>")
        .about("Rust echo")
        .arg(
            // A required positional argument that
            // must appear at least once and can be repeated.
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            // This is a flag that has only the
            // short name -n and takes no value.
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        // upon a failed parse an error will be displayed to the user and the
        // process will exit with the appropriate error code.
        .get_matches();

    // {:?} to format the debug view of the arguments.
    // {:#?} to include newlines and indentations to help
    //       me read the output. This is called pretty-printing
    // println!("{:#?}", matches);

    let text = matches.values_of_lossy("text").unwrap();
    // In Rust, if is an expression, not a statement as in Java/C.
    // An expression returns a value, but a statement does not. 
    let ending = if matches.is_present("omit_newline") {
        ""
    } else {
        "\n"
    };
    print!("{}{}", text.join("_"), ending);
}
