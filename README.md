# cota-registry-aggregator

The registry aggregator service of [CoTA](https://talk.nervos.org/t/rfc-cota-a-compact-token-aggregator-standard-for-extremely-low-cost-nfts-and-fts/6338)

[CoTA Docs](https://developer.mibao.net/docs/develop/cota/overview)

## Prerequisites

- [CoTA Syncer](https://github.com/nervina-labs/cota-nft-entries-syncer): The server to index CoTA data from CKB

> The registry aggregator and syncer share the same mysql database, and the aggregator use CoTA data from the database to provide RPC service

## Quick Start

### Manual

- Rename `.env.example` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `make build-release`
- Run with release profile: `make run-release`

### Release

```shell
RUST_LOG=info DATABASE_URL=mysql://root:passport@localhost:3306/db_name ./target/release/cota-registry-aggregator
```


### docker

```shell
# Build cota-aggregator images from the Dockerfile and run cota-aggregator via docker
docker build -t cota-registry-aggregator .
docker run -d -p 3050:3050 cota-registry-aggregator:latest

# or
docker-compose up -d --build
```

### APIs

- register_cota_cells

```shell
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "register_cota_cells",
    "params": ["0x6a8f45a094cbe050d1a612924901b11edc1bce28c0fd8d96cdc8779889f28aa8"]
}' \
| tr -d '\n' \
| curl -H 'content-type: application/json' -d @- \
http://localhost:3050
```

```shell
{
    "jsonrpc":"2.0",
    "result":{
        "block_number":4397997,
        "registry_smt_entry":"4c0200000c00000050000000010000006a8f45a094cbe050d1a612924901b11edc1bce28c0fd8d96cdc8779889f28aa8fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80100004c4f0751075bc88014ded991c0499070cae6c057bb91d4c0ecafce6213362aa9a3790b42610a000000000000000000000000000000000000000000000000000000000000004feb51f3968968d1703e277eeec7d2658681f78c25a5ff7e5761d12dfaa6d3879313066a0ad5f6bda6b14b7a086d5c6e792ebfe6a8295d56ba3f8f29abadcef116e603004f0151f5721b4c8f1df37417028122f2dd2c1119c16a8eefb15db2281eba6b1bc5bff339c2b0115623c15f01c63d079919760736164c0ac98e4d53b7a0b117221ed012004f02510168f67eb4bae3e719b844f05b4624af87c959108432f57d9a7ab50bddbbef57b60000000000000000000000000000000000000000000000000000000000000000503fa9c8d7d0ead04db0c3319a6c982b4075f354c4970f786d860b8e4d9d8981385055d41b1f09d954172d283bfdaf196b33599b717f30ca6caa733dd1cfdd0a865f5044ffc462e528ff0dd17da3f50361d5b32fb5e212dd677ee20c8684c776b4950050e25bb81682a717bf085d51335162e27fb34b0fda80afd81edbc2f14b05a608f550ed3bbc9376b8fe96ee687a270d30ac36d27941393d245c4d031c6c260127c826508a82fbce2e14ddf60900f69a0d32a590cf22dca924757ba5fc0e7bb6fa0ac700507cc710f619518d9188f231f9c04b813a6a7933c339c5092bfb40a215e803a8e1",
        "smt_root_hash":"16eee06c95fd876c674a1d757654e4becae3f60a72bd10abed3a4f8eee8a7b0e"
    },
    "id":2
}
```