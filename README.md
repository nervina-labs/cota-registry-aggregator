# cota-registry-aggregator

The aggregator of [CoTA](https://github.com/nervina-labs/ckb-cota-scripts) registry service

### Quick Start

Update `database_url` in `aggregator.toml` with your mysql url or set `DATABASE_URL` as environment variable

```shell
make build

make run

make test
```

If you set `DATABASE_URL` as environment variable, you can run as below:

```shell
DATABASE_URL=mysql://root:password@localhost:3306/db_name cargo run
```

### Usage

```shell
make build-release
RUST_LOG=info ./target/release/cota-registry-aggregator

# or
make install
RUST_LOG=info cota-registry-aggregator
```

If you set `DATABASE_URL` as environment variable, you can run as below:

```shell
make build-release
RUST_LOG=info DATABASE_URL=mysql://root:password@localhost:3306/db_name ./target/release/cota-registry-aggregator

# or
make install
RUST_LOG=info DATABASE_URL=mysql://root:password@localhost:3306/db_name cota-registry-aggregator
```

```shell
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "register_cota_cells",
    "params": ["0xea28c98f38b4a57aa81756b167bb37fa42daf67edbc9863afb8172096ed301c2"]
}' \
| tr -d '\n' \
| curl -H 'content-type: application/json' -d @- \
http://localhost:3050
```

### Deploy with docker

```shell
# Build cota-aggregator images from the Dockerfile and run cota-aggregator via docker
docker build -t cota-registry-aggregator .
docker run -d -p 3050:3050 cota-registry-aggregator:latest

# or
docker-compose up -d --build
```