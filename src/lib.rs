use std::error;
use std::fmt;

// Standard "error-boxing" Result type:
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Args {
    pub target: String,
    pub edit: Option<String>,
    pub rename: Option<String>,
    pub delete: bool,
    pub verbose: bool,
    pub quiet: bool,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ARGS: target={} edit={} rename={} delete={} verbose={} quiet={}",
            format!("\"{}\"", self.target),
            match &self.edit {
                Some(arg) => format!("\"{}\"", arg),
                None => String::from("None"),
            },
            match &self.rename {
                Some(arg) => format!("\"{}\"", arg),
                None => String::from("None"),
            },
            self.delete,
            self.verbose,
            self.quiet
        )
    }
}

impl Args {
    pub fn new(matches: clap::ArgMatches) -> Result<Args> {
        // Parse an argument with a default value (empty string):
        let target = String::from(matches.value_of("TARGET").unwrap_or_else(|| ""));

        // Parse optional arguments:
        let edit = matches.value_of("EDIT").map(String::from);
        let rename = matches.value_of("RENAME").map(String::from);
        // arg.to_string() and String::from(arg) are equivalent, but String::from
        // better communicates intent (conversion from &str to String).

        // Parse boolean flag arguments:
        let delete = matches.is_present("delete");
        let verbose = matches.is_present("verbose");
        let quiet = matches.is_present("quiet");

        // Return fully-populated args object:
        Ok(Args {
            target,
            edit,
            rename,
            delete,
            verbose,
            quiet,
        })
    }
}
