# cota-registry-aggregator

The aggregator of [CoTA](https://talk.nervos.org/t/rfc-cota-a-compact-token-aggregator-standard-for-extremely-low-cost-nfts-and-fts/6338) registry service

## Quick Start

### Manual

- Rename `.env.example` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `make build-release`
- Run with release profile: `make run-release`

### docker

```shell
# Build cota-aggregator images from the Dockerfile and run cota-aggregator via docker
docker build -t cota-registry-aggregator .
docker run -d -p 3050:3050 cota-registry-aggregator:latest

# or
docker-compose up -d --build
```

### APIs

// TODO: Add more apis

```shell
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "register_cota_cells",
    "params": ["0x8a8f45a094cbe050d1a612924901b11edc1bce28c0fd8d96cdc8779889f28aa8"]
}' \
| tr -d '\n' \
| curl -H 'content-type: application/json' -d @- \
http://localhost:3050
```
