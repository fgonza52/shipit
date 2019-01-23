use crate::git;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use std::collections::HashMap;

pub fn query(client: &super::Client, refs: &[git::Ref])
    -> Result<super::DateTime, super::Error>
{
    // Build the fragments
    let fragments = refs
        .iter()
        .enumerate()
        .map(|(i, r)| {
            format!(r##"
                alias_{}: ref(qualifiedName: "{}") {{
                    target {{
                        ... on Commit {{
                            pushedDate
                        }}
                    }}
                }}
                "##, i, r)
        })
        .fold(String::new(), |mut s, frag| {
            s.push_str(&frag);
            s
        });

    let query = format!(r##"
        query {{
            repository(owner: "tokio-rs", name: "tokio") {{
                {}
            }}
        }}"##, fragments);

    let response: Response = client.query(Request {
        query,
    })?;

    let mut times: Vec<_> = response
        .data
        .repository
        .values()
        .map(|r| {
            r.target.pushedDate
        })
        .collect();

    times.sort();

    Ok(times[0])
}

#[derive(Debug, Serialize)]
struct Request {
    query: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    repository: HashMap<String, Ref>,
}

#[derive(Debug, Deserialize)]
struct Ref {
    target: Target,
}

#[derive(Debug, Deserialize)]
struct Target {
    pushedDate: super::DateTime,
}