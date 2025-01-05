<h1 align="center">WRPT</h1>

<h4 align="center">A minimal <a href="https://portainer.io/" target="_blank">Portainer</a> cli built with <a href="https://www.rust-lang.org" target="_blank">Rust</a></h4>

<p align="center">
    <a href="https://crates.io/crates/wrpt"><img src="https://img.shields.io/crates/v/wrpt.svg" alt=""/></a>
    <a href="https://hub.docker.com/repository/docker/wahl/wrpt"><img src="https://img.shields.io/docker/v/wahl/wrpt?sort=semver&label=dockerhub&color=blue" alt=""/></a>
    <a href="https://github.com/wahl-dev/wrpt/actions?query=workflow:Tests"><img src="https://github.com/wahl-dev/wrpt/workflows/Tests/badge.svg" alt=""/></a>
    <a href="./LICENSE"><img src="https://img.shields.io/crates/l/wrpt.svg" alt=""/></a>
</p>

<p align="center">
    <a href="#about">About</a> •
    <a href="#roadmap">Roadmap</a> •
    <a href="#available-commands">Available Commands</a> •
    <a href="#docker">Docker</a> •
    <a href="#changelog">Changelog</a> •
    <a href="#license">License</a>
</p>

---

## About

WRPT is a lightweight command-line interface designed to streamline the deployment of Docker-Compose stacks on Portainer.  

While its primary focus is on stack deployment, it also provides additional features such as stack/endpoint listing and access control management (wip). WRPT is designed not only for manual usage but also for integration into CI/CD pipelines, making it a versatile tool for automating deployment workflows.

This project draws inspiration from <a href="https://gitlab.com/tortuetorche" target="_blank">@tortuetorche</a>'s work on <a href="https://gitlab.com/psuapp/psu" target="_blank">psuapp/psy</a>.  

It is also my first project written in Rust and is under **active development**, so contributions and feedback are welcome! Stay tuned for new features and improvements.

---

## Roadmap

Here are the planned enhancements and features for WRPT:  

- 🚧 **Access Control Management:** Enable stack deployments with fine-grained access control, allowing assignment to specific users and/or groups.  
- 🚧 **Comprehensive Documentation:** Write detailed usage guides, including setup instructions for integration into CI/CD pipelines on GitLab and GitHub.
- ⏳ **Automated Testing:** Write tests to ensure the reliability and stability of the tool. 
- 💭 **Kubernetes Compatibility:** Extend the tool to support Portainer deployments on Kubernetes environment.
- ✅ **Automated Release Process:** Implement CI pipelines to generate changelogs and releases automatically based on versioning and commit history.
- ✅ **Docker Image:** Create a Docker image.

### Legend
- ✅ : Completed  
- 🚧 : In progress  
- ⏳ : Pending  
- 💭 : In reflection
- ❌ : Abandoned 

---

## Available Commands

| Name                                                | Description                                               |
|-----------------------------------------------------|-----------------------------------------------------------|
| [`stack deploy`](#stack-deploy)                     | Deploy/update a stack.                                    |
| [`stack remove`](#stack-remove)                     | Remove a stack.                                           |
| [`stack list`](#stack-list)                         | List all stacks based on the current user authorizations. |
| [`stack resource-control`](#stack-resource-control) | Display the ResourceControl details of a specific stack.  |
| [`endpoint list`](#endpoint-list)                   | List endpoints.                                           |
| [`team list`](#team-list)                           | List teams.                                               |
| [`user list`](#user-list)                           | List users.                                               |
| `help`                                              | Display help message.                                     |

### Global options

| Flag  | Option                          | Description                                                                                                                                          |
|-------|---------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| -l    | `--url <URL>`                   | URL of the Portainer instance.                                                                                                                       |
| -A    | `--access-token <ACCESS_TOKEN>` | Access token of the Portainer instance. Learn how to generate an access token [here](https://docs.portainer.io/api/access#creating-an-access-token). |
|       | `--color <COLOR>`               | When to use terminal colours [default: auto] [possible values: auto, always, never].                                                                 |
| -v... |                                 | Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace.                                     |
| -q    | `--quiet`                       | Do not output any message.                                                                                                                           |
| -h    | `--help`                        | Print help.                                                                                                                                          |

### Available environment variables

| Environment variable           | Description                             |
|--------------------------------|-----------------------------------------|
| `PORTAINER_URL=URL`            | URL of the Portainer instance.          |
| `PORTAINER_ACCESS_TOKEN=TOKEN` | Access token of the Portainer instance. |

### Commands in details

#### Stack

##### Stack deploy

```
Deploy a stack

Usage: wrpt stack deploy [OPTIONS] --endpoint <ENDPOINT> --compose-file <COMPOSE_FILE> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>          Id of the environment (endpoint) that will be used
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -c, --compose-file <COMPOSE_FILE>  Path to docker compose/stack file
  -e, --env-file <ENV_FILE>          Path to a file of environment variables, to be used by the stack
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
      --prune                        Whether to prune unused containers or not
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
      --pull-image                   Force a pulling to current image with the original tag though the image is already the latest
  -h, --help                         Print help
```

##### Stack remove

```
Remove a stack

Usage: wrpt stack remove [OPTIONS] --endpoint <ENDPOINT> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>          Id of the environment (endpoint) that will be used
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

##### Stack resource-control

```
Display the ResourceControl details of a specific stack

Usage: wrpt stack resource-control [OPTIONS] --endpoint <ENDPOINT> <STACK_NAME>

Arguments:
  <STACK_NAME>  Name of the stack

Options:
  -E, --endpoint <ENDPOINT>          Id of the environment (endpoint) that will be used
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

##### Stack list

```
List all stacks based on the current user authorizations

Usage: wrpt stack list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### Endpoint

##### Endpoint list

```
List endpoints

Usage: wrpt endpoint list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### Team

##### Team list

```
List teams

Usage: wrpt team list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

#### User

##### User list

```
List users

Usage: wrpt user list [OPTIONS]

Options:
  -l, --url <URL>                    URL of the Portainer instance
  -A, --access-token <ACCESS_TOKEN>  Access token of the Portainer instance
  -v...                              Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace
  -q, --quiet                        Do not output any message
      --color <COLOR>                When to use terminal colours [default: auto] [possible values: auto, always, never]
  -h, --help                         Print help
```

---

## Docker

WRPT is also available as a Docker image for easier usage and integration. The image is hosted on Docker Hub: [wahl/wrpt](https://hub.docker.com/repository/docker/wahl/wrpt).

### Available Tags

The available tags for the Docker image can be found [here](https://hub.docker.com/repository/docker/wahl/wrpt/tags).

### Pull the Docker image

```bash
docker pull wahl/wrpt:latest
```

### Example usage

Below is an example of using the Docker image to list stacks:

```bash
docker run -it --rm \
  -e PORTAINER_URL="$PORTAINER_URL" \
  -e PORTAINER_ACCESS_TOKEN="$PORTAINER_ACCESS_TOKEN" \
  wahl/wrpt:latest stack list
```

### Notes
- Replace `$PORTAINER_URL` and `$PORTAINER_ACCESS_TOKEN` with your Portainer instance details.

---

## Changelog

The changelog is available in the [CHANGELOG.md](./CHANGELOG.md) file.

---

## License

The source code of this project is licensed under the [MIT license](https://opensource.org/license/MIT). 

See [LICENSE](LICENSE) file for reference.
