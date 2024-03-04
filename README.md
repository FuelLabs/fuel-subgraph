
# Fuel-Subgraph Setup Documentation

### Prerequisites

To build and run this project you need to have the following installed on your system:

- Rust (latest stable) – [How to install Rust](https://www.rust-lang.org/en-US/install.html)
    - Note that `rustfmt`, which is part of the default Rust installation, is a build-time requirement.
- PostgreSQL – [PostgreSQL Downloads](https://www.postgresql.org/download/)
- IPFS – [Installing IPFS](https://docs.ipfs.io/install/)
- Profobuf Compiler - [Installing Protobuf](https://grpc.io/docs/protoc-installation/)

### 1. Run IPFS
Open the terminal and run the following commands:

```bash
ipfs init
ipfs daemon
```

### 2. Configure PostgreSQL
After installing PostgreSQL run the following commands in the terminal:

```bash
initdb -D .postgres -E UTF8 --locale=C
pg_ctl -D .postgres -l logfile start
createdb graph-node
```
`Note:` In case of issues, restart the database by deleting it and then starting it again

### 3. Build Firehose-Extract
Navigate to Firehose-Extract folder and run the following command in the terminal:

```bash
cargo build
```

### 4. Start Fuel-Firehose
Ensure all arguments in `devel/fuelfire/fuelfire.yaml` are uncommented, then run the following command:

```bash
./firehose-fuel/devel/fuelfire/start.sh
```

### 5. Run Fuel-Graph-Node
Navigate to Fuel-Graph-Node folder and run the following command in the terminal:

```bash
cargo run --bin graph-node -- --config fuel_config.toml --ipfs http://127.0.0.1:5001 --node-id fuel-node-indexer
```

## Working with Fuel-Subgraphs

### 1. Install and Build Fuel-Graph-Tooling
Navigate to Fuel-Graph-Tooling folder and run:

```bash
pnpm install
pnpm build
```


### 2. Building and Deploying Fuel-Subgraphs
Initialize a new subgraph using the appropriate [.yaml](fuel-example-subgraph/subgraph.yaml) file, then navigate to the subgraph folder and execute the following commands:

```bash
./../fuel-graph-tooling/packages/cli/bin/run codegen
./../fuel-graph-tooling/packages/cli/bin/run build
./../fuel-graph-tooling/packages/cli/bin/run create my/new_sub --node http://localhost:8020/
./../fuel-graph-tooling/packages/cli/bin/run deploy -l v0.1.0 --node http://localhost:8020/ --ipfs http://localhost:5001 my/new_sub deploy -l v0.1.0 --node http://localhost:8020/ --ipfs http://localhost:5001 my/new_sub
```
#### For additional information, refer to the following documentation:
- [firehose](firehose-fuel/README.md)
- [graph-node](fuel-graph-node/README.md)
- [graph-tooling](fuel-graph-tooling/README.md)

# License
The primary license for this repo is `Apache 2.0`, see [`LICENSE`](./LICENSE).
