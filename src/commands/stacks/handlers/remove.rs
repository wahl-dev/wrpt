use crate::commands::consts;
use crate::commands::helpers::{
    construct_url, create_client, get_access_token, get_base_url, get_stack_id_from_name,
    handle_api_response,
};
use crate::commands::stacks::args::remove::StackRemoveCommand;
use crate::commands::wrpt::GlobalArgs;
use log_err::LogErrResult;
use simplelog::{debug, error, info};

pub(crate) fn handler(command: StackRemoveCommand, global_args: GlobalArgs) -> Result<(), ()> {
    debug!("command = {:?}", command);

    let base_url = get_base_url(&global_args)?;
    let access_token = get_access_token(&global_args)?;

    info!("Getting stack info...");
    let stack_id = get_stack_id_from_name(
        command.stack_name.as_str(),
        base_url.as_str(),
        access_token.as_str(),
    )?;

    if stack_id.is_none() {
        error!("Stack \"{}\" does not exist", command.stack_name);

        return Err(());
    }

    info!(
        "Stack \"{}\" exists (id = {})",
        command.stack_name,
        stack_id.unwrap_or_default()
    );

    info!("Deleting stack \"{}\"", command.stack_name);
    remove_stack(
        base_url.as_str(),
        access_token.as_str(),
        stack_id.unwrap_or_default(),
        command.endpoint,
    );

    info!("Done");

    Ok(())
}

pub(crate) fn remove_stack(base_url: &str, access_token: &str, stack_id: u32, entrypoint_id: u32) {
    let url = construct_url(
        base_url,
        consts::ENDPOINT_STACKS_REMOVE
            .replace("{id}", stack_id.to_string().as_str())
            .as_str(),
    )
    .log_expect("failed to construct url");

    debug!("request = DELETE {:?}", url.as_str());

    let response = create_client(access_token)
        .delete(url)
        .query(&[("endpointId", entrypoint_id)])
        .send()
        .log_expect("invalid response from API");

    let _ = handle_api_response(response);
}
