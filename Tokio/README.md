# Web Agent

Implement a web agent service that monitors a server.

Use asynchornous IO with [tokio](https://tokio.rs/). Read the [tutorial](https://tokio.rs/tokio/tutorial) before starting the server.

Integrate [Tokio Console](https://github.com/tokio-rs/console) for an overview of your tasks.


## Requitements

1. Work in teams
2. Use any way of solving the requests
  - read the `/proc` manually
  - use additional crates

## Routes

### `/status`

Method: GET

```json
{
    "cpus": 2,
    "memory": {
        "total": 100,
        "used": 30
    },
    "uptime": 10,
    "usage": 30
}
```

### `/cpus`

List the cpus

Method: GET

```json
[
    {
        "model": "cpu model",
        "manufacturer": "cpu manufacturer",
        "speed": 1000,
        "usage": 30
    }
]
```

### `/cpus/<cpu_number>`

List the information about one cpu

Methods: GET

```json
{
    "model": "cpu model",
    "manufacturer": "cpu manufacturer",
    "speed": 1000,
    "usage": 30
}
```

## `/processes`

Return the list of processes

```json
[
    {
        "pid": 2100,
        "ppid": 1000,
        "command": "....",
        "arguments": ["...", "..."],
        "memory": {
            "vsz": 1000,
            "rss": 300
        }
    },
]
```

> A quesry string might be supplied specifing the parameters to show. Ex: "?pid=true&memory=true" eturns only the pid and command.
> Hint: Use `Option` in the structure.

## `/processes/<pid>`

Return information about a process

```json
{
    "command": "....",
    "arguments": ["...", "..."],
    "memory": {
        "vsz": 1000,
        "rss": 300
    }
}
```

## `/processes/kill/<pid>`

Stop a process

```json
{
    "status": "ok or error",
    "erroor": 0
}
```

## `/processes/start`

Method POST

```json
{
    "command": "command to run",
    "arguments": ["argument1", "argument2"],
    "environment" {
        "variable1": "value",
        "variable2": "value",
        "variable3": "value"
    }
}
```

Response

```json
{
    "status": "ok or error",
    "stdout": "",
    "stderr": ""
    "error": 0
}
```

> Modify the route to start the process and immediately return and add abother route that ca verify the status of the process and return the partial output and error streams.
