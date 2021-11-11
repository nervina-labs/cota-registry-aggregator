# compact-registry-aggregator

The aggregator of compact-NFT registry service

### Quick Start
```shell
cargo build

cargo run
```

### Usage

```shell
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "register_compact_nft",
    "params": ["ckt1qyqd5eyygtdmwdr7ge736zw6z0ju6wsw7rssu8fcve"]
}' \
| tr -d '\n' \
| curl -H 'content-type: application/json' -d @- \
http://localhost:3030
```