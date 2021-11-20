mod resource;

use clap::{App, Arg, SubCommand};
use freechains::{ChainId, Client};

use std::error::Error;
use std::fs::File;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("seresa")
        .version("1.0")
        .author("Breno Brandão <lrabbt@gmail.com>")
        .about("Shares and searches research papers on a Freechains network.")
        .subcommand(
            SubCommand::with_name("share")
                .about("Manages and searches share forum posts.")
                .arg(
                    Arg::with_name("chain")
                        .short("c")
                        .long("chain")
                        .help("Chain name to be used as base for operation.")
                        .takes_value(true)
                        .required(true)
                        .value_name("CHAIN"),
                )
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .help("Freechains server hostname.")
                        .takes_value(true)
                        .global(true)
                        .value_name("HOST"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .help("Freechains server port.")
                        .takes_value(true)
                        .global(true)
                        .value_name("PORT"),
                )
                .subcommand(
                    SubCommand::with_name("post")
                        .about("Shares research URI.")
                        .arg(
                            Arg::with_name("signature")
                                .short("s")
                                .long("sign")
                                .help("User's private key.")
                                .takes_value(true)
                                .value_name("SIGNATURE"),
                        )
                        .arg(
                            Arg::with_name("title")
                                .short("t")
                                .long("title")
                                .help("Article's title")
                                .takes_value(true)
                                .required(true)
                                .value_name("TITLE"),
                        )
                        .arg(
                            Arg::with_name("authors")
                                .short("a")
                                .long("author")
                                .help("Article's author")
                                .takes_value(true)
                                .multiple(true)
                                .number_of_values(1)
                                .value_name("AUTHOR"),
                        )
                        .arg(
                            Arg::with_name("tags")
                                .long("tag")
                                .help("Article's tag")
                                .takes_value(true)
                                .multiple(true)
                                .number_of_values(1)
                                .value_name("TAG"),
                        )
                        .arg(
                            Arg::with_name("uri")
                                .short("u")
                                .long("uri")
                                .help("Article's URI")
                                .takes_value(true)
                                .required(true)
                                .value_name("URI"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("search")
                        .about("Searches research paper on chain.")
                        .arg(
                            Arg::with_name("strings")
                                .short("s")
                                .long("string")
                                .help(
                                    "String to be searched on article's fields.\
                            It is not case-sensitive.",
                                )
                                .takes_value(true)
                                .required(true)
                                .multiple(true)
                                .number_of_values(1)
                                .value_name("STRING"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("get-uri")
                        .about(
                            "Gets article's URI from post payload \
                    containing research paper information.",
                        )
                        .arg(
                            Arg::with_name("hash")
                                .short("H")
                                .long("hash")
                                .help("Post's hash.")
                                .takes_value(true)
                                .required(true)
                                .value_name("HASH"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("get-title")
                        .about(
                            "Gets article's title from post payload \
                    containing research paper information.",
                        )
                        .arg(
                            Arg::with_name("hash")
                                .short("H")
                                .long("hash")
                                .help("Post's hash.")
                                .takes_value(true)
                                .required(true)
                                .value_name("HASH"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("resource")
                .about("Manages resources posting and downloading.")
                .subcommand(
                    SubCommand::with_name("upload")
                        .about("Uploads resource to Freechains chain.")
                        .arg(
                            Arg::with_name("chain")
                                .short("c")
                                .long("chain")
                                .help("Chain name to be used as base for operation.")
                                .takes_value(true)
                                .required(true)
                                .value_name("CHAIN"),
                        )
                        .arg(
                            Arg::with_name("signature")
                                .short("s")
                                .long("sign")
                                .help("User's private key.")
                                .takes_value(true)
                                .value_name("SIGNATURE"),
                        )
                        .arg(
                            Arg::with_name("title")
                                .short("t")
                                .long("title")
                                .help("Article's title")
                                .takes_value(true)
                                .required(true)
                                .value_name("TITLE"),
                        )
                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Filepath of uploaded resource.")
                                .takes_value(true)
                                .required(true)
                                .value_name("FILE"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("download")
                        .about("Downloads resource from Freechains chain.")
                        .arg(
                            Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .help("Output file. If '-' or not present, prints on Stdout.")
                                .takes_value(true)
                                .value_name("FILE"),
                        )
                        .arg(
                            Arg::with_name("uri")
                                .short("u")
                                .long("uri")
                                .help("URI of the beginning of the resource.")
                                .takes_value(true)
                                .required(true)
                                .value_name("URI"),
                        ),
                ),
        )
        .get_matches();

    let host = matches.value_of("host").unwrap_or("0.0.0.0");
    let port = matches
        .value_of("port")
        .unwrap_or("8330")
        .parse::<usize>()?;

    if let Some(matches) = matches.subcommand_matches("share") {
        let chain_name = matches
            .value_of("chain")
            .expect("chain name must be defined");
        let chain_id = ChainId::new(chain_name)?;

        let addr = format!("{}:{}", host, port);
        let addr = addr.as_str();
        let client = Client::new(addr);
        let chain_client = client.chain(&chain_id);

        if let Some(matches) = matches.subcommand_matches("post") {
            let signature = matches.value_of("signature");
            let title = matches
                .value_of("title")
                .expect("article must have a title");
            let uri = matches.value_of("uri").expect("article must have an URI");
            let authors: Vec<_> = matches.values_of("authors").unwrap_or_default().collect();
            let tags: Vec<_> = matches.values_of("tags").unwrap_or_default().collect();

            seresa::share_article(
                io::stdout(),
                &chain_client,
                signature,
                title,
                &authors,
                &tags,
                uri,
            )?;
        }

        if let Some(matches) = matches.subcommand_matches("search") {
            let strings: Vec<_> = matches.values_of("strings").unwrap_or_default().collect();

            seresa::search_all(io::stdout(), &chain_client, &strings)?;
        }

        if let Some(matches) = matches.subcommand_matches("get-uri") {
            let hash = matches.value_of("hash").expect("hash must be provided");

            seresa::get_uri(io::stdout(), &chain_client, &hash)?;
        }

        if let Some(matches) = matches.subcommand_matches("get-title") {
            let hash = matches.value_of("hash").expect("hash must be provided");

            seresa::get_title(io::stdout(), &chain_client, &hash)?;
        }
    }

    if let Some(matches) = matches.subcommand_matches("resource") {
        if let Some(matches) = matches.subcommand_matches("upload") {
            let chain_name = matches
                .value_of("chain")
                .expect("chain name must be defined");
            let chain_id = ChainId::new(chain_name)?;

            let addr = format!("{}:{}", host, port);
            let addr = addr.as_str();
            let client = Client::new(addr);
            let chain_client = client.chain(&chain_id);

            let signature = matches.value_of("signature");

            let title = matches
                .value_of("title")
                .expect("article must have a title");

            let filename = matches.value_of("file").expect("file must not be empty.");
            let file = File::open(filename)?;

            resource::upload_resource(io::stdout(), file, &chain_client, signature, title)?;
        }

        if let Some(matches) = matches.subcommand_matches("download") {
            let filepath = matches.value_of("output").unwrap_or("-");
            let writer: Box<dyn io::Write> = if filepath == "-" {
                Box::new(io::stdout())
            } else {
                Box::new(File::create(filepath)?)
            };

            let uri = matches.value_of("uri").expect("download must have URI");

            let addr = format!("{}:{}", host, port);
            let addr = addr.as_str();
            let client = Client::new(addr);

            resource::download_resource(writer, &client, uri)?;
        }
    }

    Ok(())
}
