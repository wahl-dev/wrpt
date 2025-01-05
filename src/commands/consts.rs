use const_format::concatcp;

const BASE_API_PATH: &str = "/api";
pub(crate) const ENDPOINT_STACKS: &str = concatcp!(BASE_API_PATH, "/stacks");
pub(crate) const ENDPOINT_STACKS_CREATE_SWARM_STRING: &str =
    concatcp!(ENDPOINT_STACKS, "/create/swarm/string");
pub(crate) const ENDPOINT_STACKS_CREATE_STANDALONE_STRING: &str =
    concatcp!(ENDPOINT_STACKS, "/create/standalone/string");
pub(crate) const ENDPOINT_STACKS_INSPECT: &str = concatcp!(ENDPOINT_STACKS, "/{id}");
pub(crate) const ENDPOINT_STACKS_UPDATE: &str = ENDPOINT_STACKS_INSPECT;
pub(crate) const ENDPOINT_STACKS_REMOVE: &str = ENDPOINT_STACKS_INSPECT;
pub(crate) const ENDPOINT_STACKS_STOP: &str = concatcp!(ENDPOINT_STACKS_INSPECT, "/stop");

pub(crate) const ENDPOINT_ENDPOINTS: &str = concatcp!(BASE_API_PATH, "/endpoints");
pub(crate) const ENDPOINT_TEAMS: &str = concatcp!(BASE_API_PATH, "/teams");
pub(crate) const ENDPOINT_USERS: &str = concatcp!(BASE_API_PATH, "/users");
pub(crate) const ENDPOINT_ENDPOINTS_DOCKER_INFO: &str =
    concatcp!(ENDPOINT_ENDPOINTS, "/{id}/docker/info");
