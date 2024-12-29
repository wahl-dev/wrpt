
use crate::commands::consts;
use log_err::LogErrResult;
use simplelog::debug;
use crate::commands::endpoints::args::list::EndpointListCommand;
use crate::commands::endpoints::models::list::EndpointList;
use crate::commands::helpers::{build_table, construct_url, create_client, get_access_token, get_base_url, parse_api_response};
use crate::commands::wrpt::GlobalArgs;

pub(crate) fn handler(command: EndpointListCommand, global_args: GlobalArgs) {
    debug!("command = {:?}", command);
    
    let base_url = get_base_url(&global_args);
    let access_token = get_access_token(&global_args);

    let endpoints = fetch_endpoints(base_url.as_str(), access_token.as_str());

    build_table(&endpoints, None).printstd();
}

pub(crate) fn fetch_endpoints(base_url: &str, access_token: &str) -> Vec<EndpointList> {
    let url = construct_url(base_url, consts::ENDPOINT_ENDPOINTS).log_expect("failed to construct url");
    
    debug!("request = GET {:?}", url.as_str());

    let response = create_client(access_token)
        .get(url)
        .query(&[("excludeSnapshots", "true")])
        .send()
        .log_expect("invalid response from API")
    ;

    parse_api_response(response)
}
