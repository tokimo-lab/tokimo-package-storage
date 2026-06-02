# tokimo-package-storage

Shared `StorageProvider` trait and local filesystem implementation for Tokimo.

## Features

| Feature | Description |
|---|---|
| `StorageProvider` trait | Async `Send + Sync` abstraction for blob storage |
| `OpendalStorageProvider` | Local filesystem backend via [OpenDAL](https://github.com/apache/opendal) |
| `UploadOptions` | Content-type metadata for uploads |
| `StorageObject` | Object metadata (key, size) for listings |

## Usage

```rust
use tokimo_package_storage::{OpendalStorageProvider, StorageProvider};
use bytes::Bytes;
use std::path::PathBuf;

let storage = OpendalStorageProvider::new(PathBuf::from("./data"));
storage.upload("hello.txt", Bytes::from("world"), None).await?;
let data = storage.download("hello.txt").await?;
```

## Add to Cargo.toml

```toml
tokimo-package-storage = { git = "https://github.com/tokimo-lab/tokimo-package-storage" }
```

## License

MIT
