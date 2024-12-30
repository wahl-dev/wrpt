use crate::commands::consts;
use crate::commands::helpers::{
    build_table, construct_url, create_client, get_access_token, get_base_url, parse_api_response,
};
use crate::commands::teams::args::list::TeamListCommand;
use crate::commands::teams::models::list::TeamList;
use crate::commands::wrpt::GlobalArgs;
use log_err::LogErrResult;
use simplelog::debug;

pub(crate) fn handler(command: TeamListCommand, global_args: GlobalArgs) -> Result<(), ()> {
    debug!("command = {:?}", command);

    let base_url = get_base_url(&global_args)?;
    let access_token = get_access_token(&global_args)?;

    let teams = fetch_teams(base_url.as_str(), access_token.as_str())?;

    build_table(&teams, None).printstd();

    Ok(())
}

pub(crate) fn fetch_teams(base_url: &str, access_token: &str) -> Result<Vec<TeamList>, ()> {
    let url = construct_url(base_url, consts::ENDPOINT_TEAMS).log_expect("failed to construct url");

    debug!("request = GET {:?}", url.as_str());

    let response = create_client(access_token)
        .get(url)
        .send()
        .log_expect("invalid response from API");

    parse_api_response(response)
}
