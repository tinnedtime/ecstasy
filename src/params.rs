use crate::error::EcstasyError;
use clap::{Arg, Command};
use clap::AppSettings::ArgRequiredElseHelp;
use log::error;

#[derive(Clone, Debug)]
pub struct EcstasyParams {
    pub verbose: bool,
    pub debug: bool,
    pub sources: Vec<String>,
    pub ecstasy_filter: EcstasyFilter,
    pub insane: bool
}

#[derive(Clone, Debug)]
pub struct EcstasyFilter {
    pub tags: Vec<String>,
    pub subreddits: Vec<String>,
    pub pagelimit: u64
}

impl EcstasyParams {
    pub fn new() -> Result<Self, EcstasyError> {
        let matches =
            Command::new("ecstasy")
                .arg(
                    Arg::new("debug")
                        .long("debug")
                        .short('d')
                        .help("Goes through the collection processing without downloading anything")
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Display debug logs")
                )
                .arg(
                    Arg::new("sources")
                        .long("sources")
                        .short('s')
                        .help("The website to scrape. Type \"all\" for all, separate multiple with a comma.")
                        .value_name("sources")
                        .default_value("all")
                )
                .arg(
                    Arg::new("tags")
                        .long("tags")
                        .short('t')
                        .help("Define the tags you wish to scrap, separate multiple with a comma.")
                        .value_name("tags")
                )
                .arg(
                    Arg::new("subreddits")
                        .long("subreddits")
                        .short('r')
                        .help("The subreddits to scrape, seperate multiple with a comma")
                        .value_name("subreddits")
                )
                .arg(
                    Arg::new("insanity")
                        .long("insanity")
                        .short('i')
                        .help("Overrides the empty tag limitation, allowing you to scrap entire websites.")
                )
                .arg(
                    Arg::new("pagelimit")
                        .long("pagelimit")
                        .short('l')
                        .help("The maximum number of pages to download.")
                        .value_name("pagelimit")
                )
                .arg(
                    Arg::new("gui")
                        .long("gui")
                        .help("Opens a viewer where images can be reviewed one at a time before downloading.")
                )
                .get_matches();
        let verbose = matches.is_present("verbose");
        let debug = matches.is_present("debug");
        let insane = matches.is_present("insanity");
        let sources = match matches.value_of("sources") {
            Some(srcs) => {
                let mut clean = Vec::<String>::new();
                let pieces = srcs.split(',');
                for piece in pieces {
                    clean.push(piece.trim().to_owned())
                }
                clean
            }
            None => { Vec::new() }
        };
        let tags = match matches.value_of("tags") {
            Some(tags) => {
                let mut clean = Vec::<String>::new();
                let pieces = tags.split(',');
                for piece in pieces {
                    clean.push(piece.trim().to_owned())
                }
                clean.sort();
                clean
            }
            None => { Vec::new() }
        };
        let subreddits = match matches.value_of("subreddits") {
            Some(subreddits) => {
                let mut clean = Vec::<String>::new();
                let pieces = subreddits.split(',');
                for piece in pieces {
                    clean.push(piece.trim().to_owned())
                }
                clean.sort();
                clean
            }
            None => { Vec::new() }
        };
        let pagelimit: u64 = match matches.value_of("pagelimit") {
            Some(pagelimit) => {
                pagelimit
                    .parse::<u64>()
                    .unwrap_or_else(|why| {
                        error!(
                            "Failed to parse pagelimit, defaulting to maximum: {:#?}",
                            why
                        );
                        u64::MAX
                    })
            }
            None => u64::MAX
        };
        Ok(Self {
            verbose,
            debug,
            sources,
            ecstasy_filter: EcstasyFilter {
                tags,
                subreddits,
                pagelimit
            },
            insane
        })
    }
}

impl EcstasyFilter {
    pub fn is_empty(&self) -> bool {
        self.subreddits.is_empty() && self.tags.is_empty()
    }
}
