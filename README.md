<h1 align="center">WRPT</h1>

<h4 align="center">A minimal <a href="https://portainer.io/" target="_blank">Portainer</a> cli built with <a href="https://www.rust-lang.org" target="_blank">Rust</a></h4>

<p align="center">
  <a href="#available-commands">Available Commands</a> •
  <a href="#license">License</a>
</p>

---

<p align="center">
Inspired by <a href="https://gitlab.com/tortuetorche" target="_blank">@tortuetorche</a>'s work on <a href="https://gitlab.com/psuapp/psu" target="_blank">psuaapp/psy</a>
</p>

---

## Available Commands

| Name                              | Description                                               |
|-----------------------------------|-----------------------------------------------------------|
| [`stack deploy`](#stack-deploy)   | Deploy/update a stack.                                    |
| [`stack remove`](#stack-remove)   | Remove a stack.                                           |
| [`stack list`](#stack-list)       | List all stacks based on the current user authorizations. |
| [`endpoint list`](#endpoint-list) | List endpoints.                                           |
| [`help`](#help)                   | Display help message.                                     |

### Global options

| Flag  | Option                          | Description                                                                                                      |
|-------|---------------------------------|------------------------------------------------------------------------------------------------------------------|
| -l    | `--url <URL>`                   | URL of the Portainer instance.                                                                                   |
| -A    | `--access-token <ACCESS_TOKEN>` | Access token of the Portainer instance.                                                                          |
|       | `--color <COLOR>`               | When to use terminal colours [default: auto] [possible values: auto, always, never].                             |
| -v... |                                 | Increase the verbosity of messages: 1 for normal output, 2 for more verbose output, 3 for debug and 4 for trace. |
| -q    | `--quiet`                       | Do not output any message.                                                                                       |
| -h    | `--help`                        | Print help.                                                                                                      |

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

---
## License

[MIT](LICENSE) 

---

<p align="center">
    <a href="https://github.com/Wahib-L" target="_blank">@Wahib-L</a>
</p>
