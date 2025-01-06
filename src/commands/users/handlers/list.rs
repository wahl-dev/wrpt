use crate::commands::consts;
use crate::commands::helpers::{
    build_table, construct_url, create_client, get_access_token, get_base_url, parse_api_response,
};
use crate::commands::users::args::list::UserListCommand;
use crate::commands::users::models::list::User;
use crate::commands::wrpt::GlobalArgs;
use log_err::LogErrResult;
use simplelog::debug;

pub(crate) fn handler(command: UserListCommand, global_args: GlobalArgs) -> Result<(), ()> {
    debug!("command = {:?}", command);

    let base_url = get_base_url(&global_args)?;
    let access_token = get_access_token(&global_args)?;

    let users = fetch_users(
        base_url.as_str(),
        access_token.as_str(),
        global_args.insecure,
    )?;

    build_table(&users, Some(&["Id", "Username", "Role"])).printstd();

    Ok(())
}

pub(crate) fn fetch_users(
    base_url: &str,
    access_token: &str,
    insecure: bool,
) -> Result<Vec<User>, ()> {
    let url = construct_url(base_url, consts::ENDPOINT_USERS).log_expect("failed to construct url");

    debug!("request = GET {:?}", url.as_str());

    let response = create_client(access_token, insecure)
        .get(url)
        .send()
        .log_expect("invalid response from API");

    parse_api_response(response)
}
