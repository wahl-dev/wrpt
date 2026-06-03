use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{
    build_table, construct_url, get_stack_id_from_name, get_swarm_id_from_endpoint_id,
    parse_api_response, parse_env_file, CliContext,
};
use crate::commands::stacks::args::deploy::StackDeployCommand;
use crate::commands::stacks::models::deploy::{
    Stack, StackDeployStandaloneCreatePayload, StackDeploySwarmCreatePayload,
    StackDeployUpdatePayload,
};
use simplelog::{debug, info};
use std::fs;

pub(crate) fn handler(command: StackDeployCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let stack_file_content = fs::read_to_string(&command.compose_file).map_err(|e| {
        CliError::Io(format!(
            "unable to read compose file \"{}\": {}",
            command.compose_file.display(),
            e
        ))
    })?;
    debug!("stack_file_content = {:?}", stack_file_content);

    let env_file = parse_env_file(command.env_file).unwrap_or_default();
    debug!("env_file = {:?}", env_file);

    info!("Getting stack info...");
    let stack_id = get_stack_id_from_name(ctx, command.stack_name.as_str(), command.endpoint)?;

    let stack: Vec<Stack> = if stack_id.is_none() {
        info!("Stack \"{}\" does not exist", command.stack_name);

        info!("Getting Docker info...");
        let swarm_id = get_swarm_id_from_endpoint_id(ctx, command.endpoint)?;

        match swarm_id {
            Some(swarm_id) => {
                info!("Swarm cluster found : {}", swarm_id);

                info!("Preparing stack JSON...");
                let stack_create_payload = StackDeploySwarmCreatePayload {
                    env: env_file,
                    from_app_template: false,
                    name: command.stack_name.clone(),
                    stack_file_content,
                    swarm_id,
                };
                debug!("stack JSON = {:?}", stack_create_payload);

                info!("Creating Swarm stack \"{}\"", command.stack_name);
                create_stack(
                    ctx,
                    stack_create_payload,
                    command.endpoint,
                    consts::ENDPOINT_STACKS_CREATE_SWARM_STRING,
                )?
            }
            None => {
                info!("Swarm cluster not found");

                info!("Preparing stack JSON...");
                let stack_create_payload = StackDeployStandaloneCreatePayload {
                    env: env_file,
                    from_app_template: false,
                    name: command.stack_name.clone(),
                    stack_file_content,
                };
                debug!("stack JSON = {:?}", stack_create_payload);

                info!("Creating standalone stack \"{}\"", command.stack_name);
                create_stack(
                    ctx,
                    stack_create_payload,
                    command.endpoint,
                    consts::ENDPOINT_STACKS_CREATE_STANDALONE_STRING,
                )?
            }
        }
    } else {
        info!(
            "Stack \"{}\" exists (id = {})",
            command.stack_name,
            stack_id.unwrap_or_default()
        );

        info!("Preparing stack JSON...");
        let stack_update_payload = StackDeployUpdatePayload {
            env: env_file,
            stack_file_content,
            pull_image: command.pull_image,
            prune: command.prune,
        };
        debug!("stack JSON = {:?}", stack_update_payload);

        info!("Updating stack \"{}\"", command.stack_name);
        update_stack(
            ctx,
            stack_update_payload,
            stack_id.unwrap_or_default(),
            command.endpoint,
        )?
    };

    info!("Done");

    build_table(
        &stack,
        Some(&[
            "Id",
            "Name",
            "Type",
            "Status",
            "SwarmId",
            "EndpointId",
            // "ResourceControl",
            "CreationDate",
            "CreatedBy",
            "UpdateDate",
            "UpdatedBy",
        ]),
    )
    .printstd();

    Ok(())
}

pub(crate) fn create_stack<T: serde::Serialize>(
    ctx: &CliContext,
    stack_create_payload: T,
    endpoint_id: u32,
    endpoint: &str,
) -> Result<Vec<Stack>, CliError> {
    let url = construct_url(&ctx.base_url, endpoint)?;

    debug!("request = POST {:?}", url.as_str());

    let response = ctx
        .client
        .post(url)
        .json(&stack_create_payload)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    parse_api_response(response)
}

pub(crate) fn update_stack(
    ctx: &CliContext,
    stack_update_payload: StackDeployUpdatePayload,
    stack_id: u32,
    endpoint_id: u32,
) -> Result<Vec<Stack>, CliError> {
    let url = construct_url(
        &ctx.base_url,
        &consts::ENDPOINT_STACKS_UPDATE.replace("{id}", &stack_id.to_string()),
    )?;

    debug!("request = PUT {:?}", url.as_str());

    let response = ctx
        .client
        .put(url)
        .json(&stack_update_payload)
        .query(&[("endpointId", endpoint_id)])
        .send()?;

    parse_api_response(response)
}
