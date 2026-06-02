use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::stacks::handlers::list::fetch_stacks;
use crate::commands::stacks::models::deploy::EnvVar;
use crate::commands::wrpt::GlobalArgs;
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use prettytable::{cell, Cell, Row, Table};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value::Null;
use simplelog::{debug, error, warn};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::Duration;

/// Shared context holding a reusable HTTP client and resolved configuration.
pub(crate) struct CliContext {
    pub client: Client,
    pub base_url: String,
}

impl CliContext {
    pub fn from_global_args(global_args: &GlobalArgs) -> Result<Self, CliError> {
        let base_url = get_base_url(global_args)?;
        let access_token = get_access_token(global_args)?;

        if global_args.insecure {
            warn!("<yellow>SSL certificate verification is disabled</>");
        }

        let client = create_client(&access_token, global_args.insecure)?;

        Ok(CliContext { client, base_url })
    }
}

pub(crate) fn create_client(api_key: &str, insecure: bool) -> Result<Client, CliError> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-api-key"),
        HeaderValue::from_str(api_key)?,
    );

    reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(insecure)
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| CliError::Http(format!("failed to build HTTP client: {}", e)))
}

pub(crate) fn get_stack_id_from_name(
    ctx: &CliContext,
    name: &str,
) -> Result<Option<u32>, CliError> {
    let stacks = fetch_stacks(ctx)?;

    for stack in stacks {
        if stack.name.eq(name) {
            return Ok(Some(stack.id));
        }
    }

    Ok(None)
}

/// Resolves a stack by name, returning its ID or an error if it doesn't exist.
pub(crate) fn resolve_stack(ctx: &CliContext, stack_name: &str) -> Result<u32, CliError> {
    let stack_id = get_stack_id_from_name(ctx, stack_name)?;
    stack_id.ok_or_else(|| {
        error!("Stack \"{}\" does not exist", stack_name);
        CliError::Api(format!("stack \"{}\" does not exist", stack_name))
    })
}

pub(crate) fn get_swarm_id_from_endpoint_id(
    ctx: &CliContext,
    endpoint_id: u32,
) -> Result<Option<String>, CliError> {
    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_ENDPOINTS_DOCKER_INFO.replace("{id}", &endpoint_id.to_string()),
    )?;

    let response = ctx.client.get(url).send()?;

    let body = response.text().unwrap_or_default();

    let json = serde_json::from_str::<serde_json::Value>(&body).unwrap_or_default();

    let id = json
        .get("Swarm")
        .unwrap_or(&Null)
        .get("Cluster")
        .unwrap_or(&Null)
        .get("ID")
        .unwrap_or(&Null)
        .as_str();

    Ok(id.map(|s| s.to_string()))
}

pub(crate) fn get_base_url(global_args: &GlobalArgs) -> Result<String, CliError> {
    global_args
        .url
        .clone()
        .or_else(|| env::var("PORTAINER_URL").ok())
        .ok_or_else(|| {
            error!("param `url` or environment variable `PORTAINER_URL` should be set");
            CliError::Config(
                "param `url` or environment variable `PORTAINER_URL` should be set".to_string(),
            )
        })
}

pub(crate) fn get_access_token(global_args: &GlobalArgs) -> Result<String, CliError> {
    global_args
        .access_token
        .clone()
        .or_else(|| env::var("PORTAINER_ACCESS_TOKEN").ok())
        .ok_or_else(|| {
            error!("param `access-token` or environment variable `PORTAINER_ACCESS_TOKEN` should be set");
            CliError::Config(
                "param `access-token` or environment variable `PORTAINER_ACCESS_TOKEN` should be set".to_string(),
            )
        })
}

pub(crate) fn construct_url(base_url: &str, endpoint: &str) -> Result<Url, CliError> {
    let url = Url::parse(base_url)
        .map_err(|e| CliError::Config(format!("invalid base URL \"{}\": {}", base_url, e)))?;
    url.join(endpoint)
        .map_err(|e| CliError::Config(format!("invalid endpoint path \"{}\": {}", endpoint, e)))
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
            .build(),
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
    let serialized = serde_json::to_value(item).unwrap_or_default();
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
    let serialized = serde_json::to_value(item).unwrap_or_default();
    if let serde_json::Value::Object(map) = serialized {
        match columns {
            Some(cols) => cols
                .iter()
                .map(|&col| {
                    map.get(col).map_or_else(
                        || cell!(""), // If the column is missing, an empty cell is inserted.
                        process_table_value,
                    )
                })
                .collect(),
            None => map.values().map(process_table_value).collect(),
        }
    } else {
        vec![]
    }
}

fn process_table_value(value: &serde_json::Value) -> Cell {
    match value {
        serde_json::Value::Object(obj) => {
            // If the Id property exists, it is displayed
            if let Some(serde_json::Value::Number(id)) = obj.get("Id") {
                cell!(id.to_string())
            } else if let Some(serde_json::Value::String(id)) = obj.get("Id") {
                cell!(id)
            } else {
                // Otherwise, we encode in JSON
                cell!(value.to_string())
            }
        }
        serde_json::Value::Null => cell!(""),
        serde_json::Value::String(s) => cell!(s),
        _ => cell!(value.to_string()),
    }
}

pub(crate) fn handle_api_response(response: Response) -> Result<Response, CliError> {
    debug!("response = {:?}", response);

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .unwrap_or_else(|_| "<unable to read response body>".to_string());

        // Try to parse the error response as JSON
        let error_msg = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            let message = json
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("<no message>");
            let details = json
                .get("details")
                .and_then(|v| v.as_str())
                .unwrap_or("<no details>");

            error!(
                "<b>Api error</>: <i>{}</>\n<b>message</>: <i>{}</>\n<b>details</>: <i>{}</>",
                status, message, details
            );
            format!("{}: {} ({})", status, message, details)
        } else {
            // If not JSON, log the raw body
            error!(
                "<b>Api error</>: <i>{}</>\n<b>body</>: <i>{}</>",
                status, body
            );
            format!("{}: {}", status, body)
        };

        return Err(CliError::Api(error_msg));
    }

    Ok(response)
}

pub(crate) fn parse_api_response<T>(response: Response) -> Result<Vec<T>, CliError>
where
    T: DeserializeOwned,
{
    let response = handle_api_response(response)?
        .text()
        .map_err(|e| CliError::Http(format!("unable to read API response: {}", e)))?;

    debug!("response_body = {:?}", response);

    // Try to parse as a collection first
    Ok(
        serde_json::from_str::<Vec<T>>(&response).unwrap_or_else(|_| {
            // If parsing as a collection fails, try parsing as a single item
            serde_json::from_str::<T>(&response)
                .map(|item| vec![item]) // Wrap the single item in a Vec
                .unwrap_or_else(|_| {
                    warn!("error when deserializing JSON response as collection or item.");
                    vec![]
                })
        }),
    )
}

pub(crate) fn parse_env_file(file_path: Option<PathBuf>) -> Result<Vec<EnvVar>, io::Error> {
    let file = File::open(file_path.unwrap_or_default())?;
    let reader = io::BufReader::new(file);

    let mut vars = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Ignore empty lines or comments
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        if let Some((name, value)) = line.split_once('=') {
            // Strip surrounding quotes from value
            let value = value.trim();
            let value = if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                &value[1..value.len() - 1]
            } else {
                value
            };
            vars.push(EnvVar {
                name: name.trim().to_string(),
                value: value.to_string(),
            });
        }
    }

    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // --- parse_env_file tests ---

    fn write_temp_env(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn parse_env_file_basic() {
        let file = write_temp_env("KEY=value\nFOO=bar\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "KEY");
        assert_eq!(result[0].value, "value");
        assert_eq!(result[1].name, "FOO");
        assert_eq!(result[1].value, "bar");
    }

    #[test]
    fn parse_env_file_comments_and_empty_lines() {
        let file = write_temp_env("# comment\n\nKEY=value\n  # indented comment\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
    }

    #[test]
    fn parse_env_file_hash_in_value_preserved() {
        let file = write_temp_env("PASSWORD=abc#123\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "PASSWORD");
        assert_eq!(result[0].value, "abc#123");
    }

    #[test]
    fn parse_env_file_quoted_value() {
        let file = write_temp_env("KEY=\"hello world\"\nKEY2='single'\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result[0].value, "hello world");
        assert_eq!(result[1].value, "single");
    }

    #[test]
    fn parse_env_file_nonexistent_file() {
        let result = parse_env_file(Some(PathBuf::from("/tmp/nonexistent_env_file_12345")));
        assert!(result.is_err());
    }

    #[test]
    fn parse_env_file_line_without_equals() {
        let file = write_temp_env("NOEQUALS\nKEY=value\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
    }

    #[test]
    fn parse_env_file_value_with_equals() {
        let file = write_temp_env("KEY=val=ue\n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "KEY");
        assert_eq!(result[0].value, "val=ue");
    }

    #[test]
    fn parse_env_file_whitespace_trimming() {
        let file = write_temp_env("  KEY  =  value  \n");
        let result = parse_env_file(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(result[0].name, "KEY");
        assert_eq!(result[0].value, "value");
    }

    // --- construct_url tests ---

    #[test]
    fn construct_url_valid() {
        let url = construct_url("https://portainer.example.com", "/api/stacks").unwrap();
        assert_eq!(url.as_str(), "https://portainer.example.com/api/stacks");
    }

    #[test]
    fn construct_url_invalid_base() {
        let result = construct_url("not-a-url", "/api/stacks");
        assert!(result.is_err());
    }

    #[test]
    fn construct_url_with_trailing_slash() {
        let url = construct_url("https://portainer.example.com/", "/api/stacks").unwrap();
        assert_eq!(url.as_str(), "https://portainer.example.com/api/stacks");
    }

    // --- get_base_url tests ---

    #[test]
    fn get_base_url_from_arg() {
        let args = GlobalArgs {
            url: Some("https://example.com".to_string()),
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert_eq!(get_base_url(&args).unwrap(), "https://example.com");
    }

    #[test]
    fn get_base_url_missing() {
        env::remove_var("PORTAINER_URL");
        let args = GlobalArgs {
            url: None,
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert!(get_base_url(&args).is_err());
    }

    // --- get_access_token tests ---

    #[test]
    fn get_access_token_from_arg() {
        let args = GlobalArgs {
            url: None,
            access_token: Some("my-token".to_string()),
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert_eq!(get_access_token(&args).unwrap(), "my-token");
    }

    #[test]
    fn get_access_token_missing() {
        env::remove_var("PORTAINER_ACCESS_TOKEN");
        let args = GlobalArgs {
            url: None,
            access_token: None,
            insecure: false,
            verbose: 1,
            quiet: false,
            color: clap::ColorChoice::Auto,
        };
        assert!(get_access_token(&args).is_err());
    }

    // --- build_table tests ---

    #[derive(Debug, serde::Serialize)]
    struct TestItem {
        id: u32,
        name: String,
    }

    #[test]
    fn build_table_empty() {
        let items: Vec<TestItem> = vec![];
        let table = build_table(&items, None);
        assert_eq!(table.len(), 0);
    }

    #[test]
    fn build_table_with_items() {
        let items = vec![
            TestItem {
                id: 1,
                name: "foo".to_string(),
            },
            TestItem {
                id: 2,
                name: "bar".to_string(),
            },
        ];
        let table = build_table(&items, None);
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn build_table_with_column_filter() {
        let items = vec![TestItem {
            id: 1,
            name: "foo".to_string(),
        }];
        let table = build_table(&items, Some(&["name"]));
        assert_eq!(table.len(), 1);
    }

    // --- create_client tests ---

    #[test]
    fn create_client_valid() {
        let client = create_client("test-token", false);
        assert!(client.is_ok());
    }

    // --- CliError display ---

    #[test]
    fn cli_error_display() {
        let err = CliError::Config("test".to_string());
        assert!(err.to_string().contains("test"));
    }
}
