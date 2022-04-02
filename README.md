# Home Assignment #4

## About

This is the same [Amogus Terminal](https://github.com/lunakoly/NetLab3#amogus-terminal-1210) from [NetLab3](https://github.com/lunakoly/NetLab3), but with the new protocol based on OpenAPI.

Made in pair with [FenstonSingel](https://github.com/FenstonSingel).

## Testing

Run the server with:

```bash
cargo run -p server
```

The default port is 6969.

Use the `./check.sh` script to send a series of pre-defined queries.

## Implementation

The `specification.yaml` file is the spec. In addition to the responses in the spec, the server may return error 500 (Internal server error).

The `openapi_client` crate was generated with the help of the [`openapi-generator`](https://openapi-generator.tech) utility (specifically, the `rust-server` generator). The contents of the `examples/server` were copied to the `tas-server` with minimal changes + modifications to make the thing work as an Amogus Terminal implementation.

The main difference (in regard to the original protocol) is the `user/me` endpoint, allowing to check whether the current user is alive. Since there's no single connection maintained, the server can't notify the client of them being dead, so they have to check this manually over time.

## Links
- Formal requirements: https://insysnw.github.io/practice/hw/openapi-spec/
- A handy editor: https://editor.swagger.io
- The netcode generator: https://openapi-generator.tech
