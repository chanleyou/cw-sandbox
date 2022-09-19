# CosmWasm Sandbox

- [Official Docs](https://docs.cosmwasm.com/docs/1.0/)
- [Osmosis Guide](https://docs.osmosis.zone/developing/dapps/get_started/cosmwasm-testnet-manual.html)

## Deploy on Osmosis

### Optimize

This uses [rust-optimizer](https://github.com/CosmWasm/rust-optimizer).

```shell
sudo docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:0.12.6
```

- TODO: Does this work with >1 contract?
- TODO: Use version 0.12.7?

### Upload

Unlike EVM, deployment requires two steps (upload then instantiate).

```shell
osmosisd tx wasm store artifacts/$CONTRACT_NAME.wasm --from wallet --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json -b block
```

Response

```json
// insert add example here
```

Find the last event's attribute value in the JSON response. This will be the `CODE_ID` necessary for next step:

### Instantiate

```
INIT='{"count":100}'
osmosisd tx wasm instantiate $CODE_ID "$INIT" \
 --from wallet --label "my first contract" --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -b block -y --no-admin
```

### Read

```shell
QUERY='{"get_count": {}}'
osmosisd query wasm contract-state smart $CONTRACT_ADDR "QUERY" --output json
```

### Write

```shell
TRY_INCREMENT='{"increment": {}}'
osmosisd tx wasm execute $CONTRACT_ADDR "$TRY_INCREMENT" --from wallet --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -y
```
