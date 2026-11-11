# nexo-core

Núcleo compartido de **Nexo**: tipos de dominio y protocolo de sincronización
que usan la desktop y el servidor.

Este es un **proyecto independiente**. No es un workspace, no contiene UI, no
accede a bases de datos y no depende de `nexo-desktop` ni de `nexo-server`.

## Contenido

| Módulo | Qué aporta |
|---|---|
| `ids` | Identificadores tipados basados en UUIDv7 |
| `money` | Montos en unidad mínima y monedas ISO 4217 |
| `sync` | Operaciones append-only, tipos de operación y cursores |
| `error` | Errores de validación compartidos |

## Uso

Desde otro proyecto Rust, agrégalo como dependencia. Mientras los proyectos
viven en el mismo directorio de trabajo:

```toml
[dependencies]
nexo-core = { path = "../nexo-core" }
```

Cuando `nexo-core` tenga repositorio remoto, se puede consumir por Git o por
versión publicada.

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

## Desarrollo

```sh
cargo fmt
cargo test
cargo clippy --all-targets
```

## Licencia

MIT. Ver [`LICENSE`](./LICENSE).
