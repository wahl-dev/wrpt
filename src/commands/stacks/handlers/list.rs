use crate::commands::consts;
use log_err::LogErrResult;
use simplelog::{debug};
use crate::commands::helpers::{build_table, construct_url, create_client, get_access_token, get_base_url, parse_api_response};
use crate::commands::stacks::args::list::StackListCommand;
use crate::commands::stacks::models::list::StackList;
use crate::commands::wrpt::GlobalArgs;

pub(crate) fn handler(command: StackListCommand, global_args: GlobalArgs) -> Result<(), ()> {
    debug!("command = {:?}", command);
    
    let base_url = get_base_url(&global_args)?;
    let access_token = get_access_token(&global_args)?;

    let stacks = fetch_stacks(base_url.as_str(), access_token.as_str())?;

    build_table(&stacks, Some(&[
        "Id",
        "Name",
        "Type",
        "Status",
        "SwarmId",
        "EndpointId",
        "CreationDate",
        "CreatedBy",
        "UpdateDate",
        "UpdatedBy",
        // "ResourceControl",
    ])).printstd();

    Ok(())
}

pub(crate) fn fetch_stacks(base_url: &str, access_token: &str) -> Result<Vec<StackList>, ()> {
    let url = construct_url(base_url, consts::ENDPOINT_STACKS).log_expect("failed to construct url");
    
    debug!("request = GET {:?}", url.as_str());

    let response = create_client(access_token)
        .get(url)
        .send()
        .log_expect("invalid response from API")
    ;

    parse_api_response(response)
}
