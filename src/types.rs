use bytes::Bytes;
use std::path::PathBuf;

/// 上传选项。
pub struct UploadOptions {
    pub content_type: Option<String>,
}

/// 列出对象时返回的元数据。
#[allow(dead_code)]
pub struct StorageObject {
    pub key: String,
    pub size: u64,
}

/// 可插拔的对象存储抽象。
///
/// 实现者必须是 `Send + Sync`（用于跨线程共享）。
/// 所有方法均为异步，返回 `Result<T, String>`。
#[async_trait::async_trait]
pub trait StorageProvider: Send + Sync {
    /// 上传文件。
    async fn upload(&self, key: &str, body: Bytes, options: Option<UploadOptions>) -> Result<(), String>;

    /// 上传文件并返回不透明的存储 key。
    ///
    /// 对于本地存储，默认实现调用 `upload(key, ...)` 并返回传入的 key。
    /// 对于 bus 存储，override 为 server 端生成的 UUID 分片路径。
    async fn upload_opaque(&self, key: &str, body: Bytes, options: Option<UploadOptions>) -> Result<String, String> {
        self.upload(key, body, options).await?;
        Ok(key.to_string())
    }

    /// 以字节流形式下载文件。
    async fn download(&self, key: &str) -> Result<Bytes, String>;

    /// 删除文件。
    async fn delete(&self, key: &str) -> Result<(), String>;

    /// 检查文件是否存在。
    async fn exists(&self, key: &str) -> Result<bool, String>;

    /// 获取文件元数据（大小等），不下载内容。不存在时返回 None。
    async fn head(&self, key: &str) -> Result<Option<StorageObject>, String>;

    /// 列出指定前缀下的所有对象。
    async fn list(&self, prefix: Option<&str>) -> Result<Vec<StorageObject>, String>;

    /// 把 `storage_key` 映射成本机文件系统的绝对路径。
    fn local_absolute_path(&self, _key: &str) -> Option<PathBuf> {
        None
    }
}
