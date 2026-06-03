use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{construct_url, handle_api_response, resolve_stack, CliContext};
use crate::commands::stacks::args::start::StackStartCommand;
use simplelog::{debug, info};

pub(crate) fn handler(command: StackStartCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    info!("Getting stack info...");
    let stack_id = resolve_stack(ctx, &command.stack_name, command.endpoint)?;

    info!(
        "Stack \"{}\" exists (id = {})",
        command.stack_name, stack_id
    );

    info!("Starting stack \"{}\"", command.stack_name);

    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_START.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = POST {:?}", url.as_str());

    let response = ctx
        .client
        .post(url)
        .query(&[("endpointId", command.endpoint)])
        .send()?;

    handle_api_response(response)?;

    info!("Done");

    Ok(())
}
