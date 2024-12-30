use const_format::concatcp;

const BASE_API_PATH: &str = "/api";
pub(crate) const ENDPOINT_STACKS: &str = concatcp!(BASE_API_PATH, "/stacks");
pub(crate) const ENDPOINT_STACKS_REMOVE: &str = concatcp!(ENDPOINT_STACKS, "/{id}");
pub(crate) const ENDPOINT_STACKS_CREATE_SWARM_STRING: &str =
    concatcp!(ENDPOINT_STACKS, "/create/swarm/string");
pub(crate) const ENDPOINT_STACKS_CREATE_STANDALONE_STRING: &str =
    concatcp!(ENDPOINT_STACKS, "/create/standalone/string");
pub(crate) const ENDPOINT_STACKS_UPDATE: &str = concatcp!(ENDPOINT_STACKS, "/{id}");

pub(crate) const ENDPOINT_ENDPOINTS: &str = concatcp!(BASE_API_PATH, "/endpoints");
pub(crate) const ENDPOINT_ENDPOINTS_DOCKER_INFO: &str =
    concatcp!(ENDPOINT_ENDPOINTS, "/{id}/docker/info");
