# TOTP

Just having fun generating TOTP
You pass the secret, you generate.

**Note:** The secret is basically generated and tested from [IT Tools](https://github.com/CorentinTh/it-tools)
```shell
# Generate a new code from secret
cargo run -- J7BOMUB5JGYWTOBT generate

# Verify the code `123456` against the secret for now
cargo run -- J7BOMUB5JGYWTOBT verify 123456
```