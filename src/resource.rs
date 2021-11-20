use freechains::{ChainClient, ChainId, Client, Connect};
use serde::{Deserialize, Serialize};
use seresa::Error::{InputError, InvalidContentError, LowReputationError};

use std::io::prelude::*;
use std::io::BufReader;

const CONTENT_BLOCK_SIZE: usize = 80 * 1024;

#[derive(Debug, Deserialize, Serialize)]
struct Resource {
    /// Resource title.
    title: String,

    /// Base64 representation of the resource block.
    content: String,

    /// Previous block of the resource, [None] if firsr block.
    prev: Option<String>,
}

pub fn upload_resource<T>(
    mut w: impl Write,
    r: impl Read,
    chain_client: &ChainClient<T>,
    signature: Option<&str>,
    title: &str,
) -> Result<(), seresa::Error>
where
    T: Connect,
{
    let mut reader = BufReader::new(r);

    let mut prev: Option<String> = None;
    let mut end = false;
    while !end {
        let mut buf = [0; CONTENT_BLOCK_SIZE];
        let mut bytes_read = 0;
        while bytes_read < CONTENT_BLOCK_SIZE {
            let n = reader.read(&mut buf[bytes_read..])?;
            bytes_read += n;

            if n == 0 {
                end = true;
                break;
            }
        }

        let title = String::from(title);
        let content = base64::encode(&buf[..bytes_read]);
        let resource = Resource {
            title,
            content,
            prev,
        };

        let payload = serde_json::to_vec(&resource)?;
        let hash = chain_client.post(signature, false, &payload)?;
        writeln!(w, "{}", hash)?;

        prev = Some(format!("fchs:{}", hash));
    }

    Ok(())
}

pub fn download_resource<T>(
    mut w: impl Write,
    client: &Client<T>,
    uri: &str,
) -> Result<(), seresa::Error>
where
    T: Connect,
{
    let (chain, mut post) = parse_uri(uri)?;
    let mut chain = chain.ok_or(InputError(String::from("missing chains name on URI")))?;

    let mut rord_content: Vec<(ChainId, String)> = Vec::new();

    loop {
        let chain_id = ChainId::new(&chain).or_else(|c| {
            Err(InputError(format!(
                "invalid 'fchs' URI format, invalid chain name \"{}\"",
                c
            )))
        })?;

        if !client.chains()?.contains(&chain_id) {
            return Err(InputError(format!(
                "invalid 'fchs' URI, chain \"{}\" not on Freechains node",
                chain_id
            )));
        }

        let chain_client = client.chain(&chain_id);
        let reps = chain_client.reputation(&post)?;
        if reps < -3 {
            return Err(LowReputationError(chain.clone(), post.clone()));
        }

        rord_content.push((chain_id, post.clone()));

        let payload = chain_client.payload(&post, None)?;
        let resource_block: Resource = serde_json::from_slice(&payload)?;

        match resource_block.prev {
            None => break,
            Some(prev) => {
                let (c, p) = parse_uri(&prev)?;

                if let Some(c) = c {
                    chain = c;
                }
                post = p;
            }
        }
    }

    rord_content.reverse();
    for (c, p) in rord_content {
        let chain_client = client.chain(&c);

        let payload = chain_client.payload(&p, None)?;
        let resource_block: Resource = serde_json::from_slice(&payload)?;

        let content = base64::decode(resource_block.content)
            .or(Err(InvalidContentError(c.to_string(), p.clone())))?;

        w.write_all(&content)?;
    }

    Ok(())
}

fn parse_uri(uri: &str) -> Result<(Option<String>, String), seresa::Error> {
    let (scheme, uri) = uri
        .split_once(':')
        .ok_or(InputError(String::from("no scheme specified on URI")))?;
    if scheme != "fchs" {
        return Err(InputError(String::from("invalid URI scheme")));
    }

    match uri.rsplit_once(':') {
        None => Ok((None, String::from(uri))),
        Some((uri, post)) => {
            if uri.len() == 0 {
                return Err(InputError(String::from(
                    "invalid 'fchs' URI format, missing chain name",
                )));
            }
            let chain = match uri.split_once(':') {
                None => uri,
                Some((chain, _)) => chain,
            };

            Ok((Some(String::from(chain)), String::from(post)))
        }
    }
}
