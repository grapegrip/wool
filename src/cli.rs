use clap::{App, Arg, ArgMatches};

pub fn get_cli_matches<'a>() -> ArgMatches<'static> {
    App::new("ynot")
        .version("0.1")
        .author("m")
        .about("See https://github.com/grapegrip/wool")
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
                .help("Export html"),
        )
        .arg(
            Arg::with_name("no-preview-frame")
                .short("n")
                .long("no-preview-frame")
                .help("Don't render the preview frame"),
        )
        .arg(
            Arg::with_name("highlight")
                .short("s")
                .long("highlight")
                .help("Syntax highlighting"),
        )
        .arg(
            Arg::with_name("browser")
                .short("b")
                .long("browser")
                .help("Open in browser"),
        )
        .arg(
            Arg::with_name("katex")
                .short("k")
                .long("katex")
                .help("Include katex in rendering"),
        )
        .arg(Arg::with_name("outfile").help("Sets the output file to use"))
        .get_matches()
}
