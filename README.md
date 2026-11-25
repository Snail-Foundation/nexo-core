# nexo-core

Shared core for **Nexo**: domain types and the synchronization protocol used by
the Nexo server and the archived desktop clients.

This is an **independent project**. It is not a workspace, does not contain UI,
does not access databases, and does not depend on `nexo-desktop` or
`nexo-server`.

## Contents

| Module | Purpose |
|---|---|
| `ids` | Typed identifiers based on UUIDv7 |
| `money` | Minor-unit amounts and ISO 4217 currencies |
| `restaurant` | Order types, order and line statuses, and the optional global IVA |
| `sync` | Append-only operations, operation kinds, and cursors |
| `error` | Shared validation errors |

## Usage

From another Rust project, add it as a dependency.

Outside the Snail workspace, use the Git repository:

```toml
[dependencies]
nexo-core = { git = "https://github.com/Snail-Foundation/nexo-core.git" }
```

Inside the Snail workspace, where the projects are cloned side by side, use the
local path:

```toml
[dependencies]
nexo-core = { path = "../nexo-core" }
```

The crate is not published on crates.io (`publish = false`). The server and the
archived desktop projects use the local path form, so clone `nexo-core` next to
them before building:

```sh
git clone https://github.com/Snail-Foundation/nexo-core.git
git clone https://github.com/Snail-Foundation/nexo-server.git
# nexo-core and nexo-server must share the same parent directory
```

```rust
use nexo_core::{DeviceId, Operation, OperationKind, Uuid};
use serde_json::json;

let operation = Operation::new(
    DeviceId::new(),
    None,
    OperationKind::SaleCreated,
    Uuid::now_v7(),
    json!({ "total_minor_units": 1500 }),
    1_700_000_000_000,
);

operation.validate().unwrap();
```

## Development

```sh
cargo fmt
cargo test
cargo clippy --all-targets
```

## License

MIT. See [`LICENSE`](./LICENSE).
