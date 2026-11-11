# nexo-core

Shared core for **Nexo**: domain types and the synchronization protocol used by
the desktop and the server.

This is an **independent project**. It is not a workspace, does not contain UI,
does not access databases, and does not depend on `nexo-desktop` or
`nexo-server`.

## Contents

| Module | Purpose |
|---|---|
| `ids` | Typed identifiers based on UUIDv7 |
| `money` | Minor-unit amounts and ISO 4217 currencies |
| `sync` | Append-only operations, operation kinds, and cursors |
| `error` | Shared validation errors |

## Usage

From another Rust project, add it as a dependency. While the projects live in
the same working directory:

```toml
[dependencies]
nexo-core = { path = "../nexo-core" }
```

Once `nexo-core` has a remote repository, it can be consumed via Git or a
published release.

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
