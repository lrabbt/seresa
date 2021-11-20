use freechains::{ChainClient, Connect};
use serde::{Deserialize, Serialize};

use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Share {
    /// Title of the article.
    title: String,

    /// Authors of the article.
    authors: Vec<String>,

    /// Tags of the article.
    tags: Vec<String>,

    /// Article URI.
    uri: String,
}

#[derive(Debug)]
pub enum Error {
    SerdeJsonError(serde_json::Error),
    FreechainsError(freechains::ClientError),
    IoError(io::Error),
    InputError(String),
    InvalidContentError(String, String),
    LowReputationError(String, String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SerdeJsonError(e) => e.fmt(f),
            Error::FreechainsError(e) => e.fmt(f),
            Error::IoError(e) => e.fmt(f),
            Error::InputError(e) => write!(f, "Invalid input: {}", e),
            Error::InvalidContentError(c, p) => {
                write!(f, "Invalid content on chain \"{}\", post \"{}\"", c, p)
            }
            Error::LowReputationError(c, p) => write!(
                f,
                "Low reputation content on chain \"{}\", post \"{}\"",
                c, p
            ),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl From<freechains::ClientError> for Error {
    fn from(e: freechains::ClientError) -> Self {
        Error::FreechainsError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

pub fn share_article<T: Connect>(
    mut w: impl io::Write,
    chain_client: &ChainClient<T>,
    signature: Option<&str>,
    title: &str,
    authors: &[&str],
    tags: &[&str],
    uri: &str,
) -> Result<(), Error>
where
    T: Connect,
{
    let title = String::from(title);
    let authors: Vec<_> = authors.iter().map(|&a| String::from(a)).collect();
    let tags: Vec<_> = tags.iter().map(|&t| String::from(t)).collect();
    let uri = String::from(uri);
    let share = Share {
        title,
        authors,
        tags,
        uri,
    };

    let payload = serde_json::to_vec(&share)?;

    let hash = chain_client.post(signature, false, &payload)?;
    writeln!(w, "{}", hash)?;

    Ok(())
}

pub fn search_all<T>(
    mut w: impl io::Write,
    chain_client: &ChainClient<T>,
    strings: &[&str],
) -> Result<(), Error>
where
    T: Connect,
{
    let consensus = chain_client.consensus()?;

    let strings: Vec<_> = strings.iter().map(|s| s.to_lowercase()).collect();
    for hash in consensus {
        let payload = chain_client.payload(&hash, None)?;

        if let Ok(share) = serde_json::from_slice::<Share>(&payload) {
            for s in &strings {
                let s = s.as_str();
                let found = share.title.to_lowercase().contains(&s)
                    || share
                        .authors
                        .iter()
                        .find(|&a| a.to_lowercase().contains(&s))
                        .is_some()
                    || share
                        .tags
                        .iter()
                        .find(|&t| t.to_lowercase().contains(&s))
                        .is_some();
                if found {
                    writeln!(w, "{}", hash)?;
                    break;
                }
            }
        }
    }

    Ok(())
}

pub fn get_uri<T>(
    mut w: impl io::Write,
    chain_client: &ChainClient<T>,
    hash: &str,
) -> Result<(), Error>
where
    T: Connect,
{
    let payload = chain_client.payload(hash, None)?;

    let share: Share = serde_json::from_slice(&payload)?;
    writeln!(w, "{}", share.uri)?;

    Ok(())
}

pub fn get_title<T>(
    mut w: impl io::Write,
    chain_client: &ChainClient<T>,
    hash: &str,
) -> Result<(), Error>
where
    T: Connect,
{
    let payload = chain_client.payload(hash, None)?;

    let share: Share = serde_json::from_slice(&payload)?;
    writeln!(w, "{}", share.title)?;

    Ok(())
}
