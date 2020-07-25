# Espacio Binario Poscast API

API that provide the list of podcast of the **Espacio Binario Poscast**. Part of the [binary-coffee.dev](binary-coffee.dev) project.

## Start project

```
cargo run
```

### Environment variables

Locally you can create the file `rocket.toml` and define the environment variables in the following format:

```
[development]
ROCKET_VAR_NAME = <value>
...

[staging]
ROCKET_VAR_NAME = <value>
...

[production]
ROCKET_VAR_NAME = <value>
...
```
Depending of the environment you are runing, is the variables that are going to being use.

**Rocket**

- `ROCKET_ADDRESS`: Default value `localhost`. **(optional)**
- `ROCKET_PORT`: Default value `8000`. **(optional)** 
- `ROCKET_WORKERS`: Default value `[number of cpus * 2]`. **(optional)** 
- `ROCKET_KEEP_ALIVE`: Default value `5`. **(optional)** 
- `ROCKET_LOG`: Default value `normal`. **(optional)** 
- `ROCKET_SECRET_KEY`: Default value `[randomly generated at launch]`. **(optional)** 
- `ROCKET_LIMITS`: Default value `{ forms = 32768 }`. **(optional)** 

**Project**

- `ROCKET_DB_HOST`: Default value `27018`.
- `ROCKET_DB_PORT`: Default value `localhost`.

## Start project with docker



## Build project

```
cargo build
```

> Note: add `--release` flag if you want to build the project in production.

## Contributing

Any contribution is welcome
