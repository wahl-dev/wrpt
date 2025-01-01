use crate::commands::consts;
use crate::commands::helpers::{
    build_table, construct_url, create_client, get_access_token, get_base_url,
    get_stack_id_from_name, get_swarm_id_from_endpoint_id, parse_api_response, parse_env_file,
};
use crate::commands::stacks::args::deploy::StackDeployCommand;
use crate::commands::stacks::models::deploy::{
    Stack, StackDeployStandaloneCreatePayload, StackDeploySwarmCreatePayload,
    StackDeployUpdatePayload,
};
use crate::commands::wrpt::GlobalArgs;
use log_err::LogErrResult;
use simplelog::{debug, info};
use std::fs;

pub(crate) fn handler(command: StackDeployCommand, global_args: GlobalArgs) -> Result<(), ()> {
    debug!("command = {:?}", command);

    let base_url = get_base_url(&global_args)?;
    let access_token = get_access_token(&global_args)?;

    let stack_file_content =
        fs::read_to_string(command.compose_file).log_expect("Unable to read `compose-file`");
    debug!("stack_file_content = {:?}", stack_file_content);

    let env_file = parse_env_file(command.env_file).unwrap_or_default();
    debug!("env_file = {:?}", env_file);

    info!("Getting stack info...");
    let stack_id = get_stack_id_from_name(
        command.stack_name.as_str(),
        base_url.as_str(),
        access_token.as_str(),
    )?;

    let stack: Vec<Stack> = if stack_id.is_none() {
        info!("Stack \"{}\" does not exist", command.stack_name);

        info!("Getting Docker info...");
        let swarm_id = get_swarm_id_from_endpoint_id(
            command.endpoint,
            base_url.as_str(),
            access_token.as_str(),
        );

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
                    base_url.as_str(),
                    access_token.as_str(),
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
                    base_url.as_str(),
                    access_token.as_str(),
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
            base_url.as_str(),
            access_token.as_str(),
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
    base_url: &str,
    access_token: &str,
    stack_create_payload: T,
    entrypoint_id: u32,
    endpoint: &str,
) -> Result<Vec<Stack>, ()> {
    let url = construct_url(base_url, endpoint).log_expect("failed to construct url");

    debug!("request = POST {:?}", url.as_str());

    let response = create_client(access_token)
        .post(url)
        .json(&stack_create_payload)
        .query(&[("endpointId", entrypoint_id)])
        .send()
        .log_expect("invalid response from API");

    parse_api_response(response)
}

pub(crate) fn update_stack(
    base_url: &str,
    access_token: &str,
    stack_update_payload: StackDeployUpdatePayload,
    stack_id: u32,
    entrypoint_id: u32,
) -> Result<Vec<Stack>, ()> {
    let url = construct_url(
        base_url,
        consts::ENDPOINT_STACKS_UPDATE
            .replace("{id}", stack_id.to_string().as_str())
            .as_str(),
    )
    .log_expect("failed to construct url");

    debug!("request = PUT {:?}", url.as_str());

    let response = create_client(access_token)
        .put(url)
        .json(&stack_update_payload)
        .query(&[("endpointId", entrypoint_id)])
        .send()
        .log_expect("invalid response from API");

    parse_api_response(response)
}
