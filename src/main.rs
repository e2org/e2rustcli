extern crate clap;

use clap::clap_app;

use e2rustcli::Args;

fn main() {
    // Parse arguments from command line via https://github.com/clap-rs/clap
    let args = Args::new(
        clap_app!(e2rustcli => // YOUR CLI TOOL NAME HERE
            // Take config values directly from Cargo.toml to keep them in sync:
            (version: env!("CARGO_PKG_VERSION"))
            (author: env!("CARGO_PKG_AUTHORS"))
            (about: env!("CARGO_PKG_DESCRIPTION"))

            // Positional argument:
            (@arg TARGET: "something to operate on")
            // (@arg TARGET: default_value("foo") "something to operate on")
            // to provide a default value.

            // Keyword arguments:
            (@arg EDIT: -e --edit +takes_value
             "edit something")
            (@arg RENAME: -r --rename +takes_value
             "rename something")

            // Boolean arguments (flags):
            (@arg delete: -d --delete
             "delete something")
            (@arg verbose: -v --verbose
             "log info + debug output to terminal")
            (@arg quiet: -q --quiet
             "suppress all info + debug output")
        )
        // Args constructor accepts a clap::ArgMatches object:
        .get_matches(),
    )
    // On any arg parse issue, print the error and exit immediately:
    .unwrap_or_else(|error| panic!("error: {:?}", error));

    // If verbose mode requested, print info line with argument values.
    // Formatting handled via Args::fmt -- implementation of Display trait.
    if args.verbose {
        println!("{}", args);
    }
}
