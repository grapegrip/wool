use clap::{App, Arg, ArgMatches};

pub fn get_cli_matches<'a>() -> ArgMatches<'static> {
    App::new("ynot")
        .version("0.1")
        .author("m")
        .about("readme")
        .arg(
            Arg::with_name("infile")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("export-flag")
                .short("e")
                .long("export")
                .help("export html"),
        )
        .arg(
            Arg::with_name("no-preview-frame")
                .short("n")
                .long("no-preview-frame")
                .help("Don't render the preview frame"),
        )
        .arg(
            Arg::with_name("highlight")
                .short("h")
                .long("highlight")
                .help("include syntax highlighting"),
        )
        .arg(Arg::with_name("outfile").help("tmp"))
        .get_matches()
}
