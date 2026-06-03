use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{
    build_table, construct_url, parse_api_response, resolve_stack, CliContext,
};
use crate::commands::stacks::args::resource_control::StackResourceControlCommand;
use crate::commands::stacks::models::deploy::Stack;
use simplelog::{debug, error, info};

pub(crate) fn handler(
    command: StackResourceControlCommand,
    ctx: &CliContext,
) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    info!("Getting stack info...");
    let stack_id = resolve_stack(ctx, &command.stack_name, command.endpoint)?;

    info!(
        "Stack \"{}\" exists (id = {})",
        command.stack_name, stack_id
    );

    info!(
        "Display the ResourceControl details of stack \"{}\"",
        command.stack_name
    );
    let stack = inspect_stack(ctx, stack_id, command.endpoint)?;

    let stack = stack.first().ok_or_else(|| {
        CliError::Api(format!(
            "no data returned for stack \"{}\"",
            command.stack_name
        ))
    })?;
    let resource_control = stack.resource_control.as_ref().ok_or_else(|| {
        error!("Stack \"{}\" has no ResourceControl", command.stack_name);
        CliError::Api(format!(
            "stack \"{}\" has no ResourceControl",
            command.stack_name
        ))
    })?;

    build_table(&[resource_control], None).printstd();

    Ok(())
}

pub(crate) fn inspect_stack(
    ctx: &CliContext,
    stack_id: u32,
    endpoint_id: u32,
) -> Result<Vec<Stack>, CliError> {
    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_INSPECT.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = GET {:?}", url.as_str());

    let response = ctx
        .client
        .get(url)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    parse_api_response(response)
}
