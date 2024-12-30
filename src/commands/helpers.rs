use std::env;
use crate::commands::consts;
use log_err::LogErrResult;
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::Value::Null;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use prettytable::{cell, Cell, Row, Table};
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use reqwest::blocking::Response;
use reqwest::Url;
use serde::de::DeserializeOwned;
use simplelog::{debug, error, warn};
use crate::commands::stacks::handlers::list::fetch_stacks;
use crate::commands::stacks::models::deploy::EnvVar;
use crate::commands::wrpt::GlobalArgs;

pub(crate) fn create_client(api_key: &str) -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                HeaderName::from_static("x-api-key"),
                HeaderValue::from_str(api_key).unwrap(),
            );
            headers
        })
        .build()
        .unwrap()
}

pub(crate) fn get_stack_id_from_name(name: &str, base_url: &str, access_token: &str) -> Result<Option<u32>, ()> {
    let stacks = fetch_stacks(base_url, access_token)?;

    for stack in stacks {
        if stack.name.eq(name)  {
            return Ok(Some(stack.id));
        }
    }

    Ok(None)
}

pub(crate) fn get_swarm_id_from_endpoint_id(
    endpoint_id: u32,
    url: &str,
    access_token: &str,
) -> Option<String> {
    let mut url = url.to_string();
    url.push_str(
        consts::ENDPOINT_ENDPOINTS_DOCKER_INFO
            .replace("{id}", endpoint_id.to_string().as_str())
            .as_str(),
    );

    let response = create_client(access_token).get(url).send();

    let body = response
        .log_expect("invalid response from API")
        .text()
        .unwrap_or_default();

    let json = serde_json::from_str::<serde_json::Value>(body.as_str()).unwrap_or_default();

    let id = json
        .get("Swarm")
        .unwrap_or(&Null)
        .get("Cluster")
        .unwrap_or(&Null)
        .get("ID")
        .unwrap_or(&Null)
        .as_str();

    if id.is_some() {
        return Some(id?.to_string());
    }

    None
}

pub(crate) fn get_base_url(global_args: &GlobalArgs) -> Result<String, ()> {
    match global_args.url.clone().or_else(|| env::var("PORTAINER_URL").ok()) {
        None => {
            error!("param `url` or environment variable `PORTAINER_URL` should be set");
            Err(())
        },
        Some(base_url) => Ok(base_url),
    }
}

pub(crate) fn get_access_token(global_args: &GlobalArgs) -> Result<String, ()> {
    match global_args.access_token.clone().or_else(|| env::var("PORTAINER_ACCESS_TOKEN").ok()) {
        None => {
            error!("param `access-token` or environment variable `PORTAINER_ACCESS_TOKEN` should be set");
            Err(())
        },
        Some(base_url) => Ok(base_url),
    }
}

pub(crate) fn construct_url(base_url: &str, endpoint: &str) -> Result<Url, String> {
    let url = Url::parse(base_url).map_err(|_| "invalid base URL".to_string())?;
    url.join(endpoint).map_err(|_| "invalid endpoint path".to_string())
}

pub(crate) fn build_table<T>(items: &[T], columns: Option<&[&str]>) -> Table
where
    T: serde::Serialize,
{
    let mut table = Table::new();
    table.set_format(
        FormatBuilder::new()
            .padding(1, 1)
            .separator(LinePosition::Title, LineSeparator::default())
            .build()
    );

    if let Some(first_item) = items.first() {
        let headers = extract_headers(first_item, columns);
        table.set_titles(Row::new(headers.iter().map(|h| cell!(h)).collect()));
    }

    for item in items {
        let row = extract_row(item, columns);
        table.add_row(Row::new(row));
    }

    table
}

fn extract_headers<T>(item: &T, columns: Option<&[&str]>) -> Vec<String>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(item).log_expect("failed to serialize item");
    if let serde_json::Value::Object(map) = serialized {
        match columns {
            Some(cols) => cols.iter().map(|&col| col.to_string()).collect(),
            None => map.keys().map(|key| key.to_string()).collect(),
        }
    } else {
        vec![]
    }
}

fn extract_row<T>(item: &T, columns: Option<&[&str]>) -> Vec<Cell>
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(item).log_expect("failed to serialize item");
    if let serde_json::Value::Object(map) = serialized {
        match columns {
            Some(cols) => cols
                .iter()
                .map(|&col| {
                    map.get(col).map_or_else(|| cell!(""), |value| {
                        match value {
                            serde_json::Value::String(s) => cell!(s),
                            _ => cell!(value.to_string()),
                        }
                    })
                })
                .collect(),
            None => map
                .values()
                .map(|value| match value {
                    serde_json::Value::String(s) => cell!(s),
                    _ => cell!(value.to_string()),
                })
                .collect(),
        }
    } else {
        vec![]
    }
}

pub(crate) fn handle_api_response(response: Response) -> Result<Response, ()> {
    debug!("response = {:?}", response);
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_else(|_| "<unable to read response body>".to_string());

        // Try to parse the error response as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("<no message>");
            let details = json.get("details").and_then(|v| v.as_str()).unwrap_or("<no details>");
            
            error!("<b>Api error</>: <i>{}</>\n<b>message</>: <i>{}</>\n<b>details</>: <i>{}</>", status, message, details);
        } else {
            // If not JSON, log the raw body
            error!("<b>Api error</>: <i>{}</>\n<b>body</>: <i>{}</>", status, body);
        }
        
        return Err(());
    }
    
    Ok(response)
}

pub(crate) fn parse_api_response<T>(response: Response) -> Result<Vec<T>, ()>
where
    T: DeserializeOwned,
{
    let response = handle_api_response(response)?
    .text()
    .unwrap_or_else(|_| {
        warn!("unable to read API response");

        String::new()
    });

    debug!("response_body = {:?}", response);

    // Try to parse as a collection first
    Ok(serde_json::from_str::<Vec<T>>(&response).unwrap_or_else(|_| {
        // If parsing as a collection fails, try parsing as a single item
        serde_json::from_str::<T>(&response)
            .map(|item| vec![item]) // Wrap the single item in a Vec
            .unwrap_or_else(|_| {
                warn!("error when deserializing JSON response as collection or item.");
                vec![]
            })
    }))
}

pub (crate) fn parse_env_file(file_path: Option<PathBuf>) -> Result<Vec<EnvVar>, io::Error> {
    let file = File::open(file_path.unwrap_or_default())?;
    let reader = io::BufReader::new(file);

    let mut vars = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Ignore empty lines or comments
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        // Remove inline comments
        let line = match line.split_once('#') {
            Some((code, _)) => code.trim().to_string(),
            None => line,
        };

        if let Some((name, value)) = line.split_once('=') {
            vars.push(EnvVar {
                name: name.trim().to_string(),
                value: value.trim().to_string(),
            });
        }
    }

    Ok(vars)
}

// fn main() {
//     let file_path = ".env"; // Remplacez par le chemin de votre fichier .env
//
//     match parse_env_file(file_path) {
//         Ok(env_vars) => {
//             let json_output = json!(env_vars);
//             println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
//         }
//         Err(e) => eprintln!("Erreur lors de la lecture du fichier .env : {}", e),
//     }
// }
