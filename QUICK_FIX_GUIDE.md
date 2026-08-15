# ⚡ 快速修复指南 - 24 小时恢复编译

**目标**: 从 37 个错误 → 0 个错误  
**时间**: 6-8 小时  
**难度**: 中等  

---

## 📋 修复清单

### ✅ 优先级顺序

```
优先级 1 (1-2h)  → 删除混入的评论代码 (repo_adapter/src/fs/)
优先级 2 (1-2h)  → 修复 Port trait 定义 (port/src/fs/)
优先级 3 (4-6h)  → 完成 Repository 实现 (repository/src/cola_fs/)
优先级 4 (0.25h) → 补充 Cargo.toml 依赖
```

---

## 🔧 优先级 1: 删除混入的评论代码 (1-2 小时)

### 问题文件

```
❌ repo_adapter/src/fs/bucket/get.rs
❌ repo_adapter/src/fs/media/{add,check,del,get,list}.rs
```

### 修复步骤

**步骤 1.1**: 打开 `repo_adapter/src/fs/bucket/get.rs`

找到这样的错误代码:
```rust
// ❌ 错误：评论方法不应该在这里
impl BucketGetPort for BucketGetAdapter {
    async fn get_comment_by_user_id(...) -> Result<...> { }
    async fn get_comment_by_video(...) -> Result<...> { }
}
```

替换为:
```rust
// ✅ 正确：bucket 的获取方法
impl BucketGetPort for BucketGetAdapter {
    async fn get_bucket_by_id(&self, id: i64) -> Result<BucketEntity> {
        todo!()
    }
    async fn get_bucket_by_app_id(&self, app_id: &str) -> Result<BucketEntity> {
        todo!()
    }
}
```

**步骤 1.2**: 清理 media 目录各文件

| 文件 | 应该实现 | 需要删除 |
|------|---------|---------|
| add.rs | create_media() | send_comment() |
| check.rs | check_media_exists() | check_health(), is_owner() |
| del.rs | delete_media() | single_delete() |
| get.rs | get_media_by_id() | get_comment_by_user_id() |
| list.rs | list_user_medias() | get_my_like_record() |

**验证**:
```bash
cargo build -p repo_adapter 2>&1 | grep -c "error\[E0407\]"
# 应该输出: 0
```

---

## 🔧 优先级 2: 修复 Port trait 定义 (1-2 小时)

### 问题文件

```
❌ port/src/fs/bucket/get.rs
❌ port/src/fs/media/{add,check,del,get}.rs
```

### 修复步骤

删除所有混入的评论方法:

```rust
// ❌ 删除这些不属于 BucketGetPort 的:
async fn get_comment_by_user_id(...);
async fn get_comment_by_video(...);

// ✅ 保留这些:
async fn get_bucket_by_id(...);
async fn get_bucket_by_app_id(...);
```

**验证**:
```bash
cargo build -p port 2>&1 | grep -c "error\["
# 应该输出: 0
```

---

## 🔧 优先级 3: 完成 Repository 实现 (4-6 小时)

### 缺失的方法

FileRepo 需要这 12+ 个方法:

```rust
pub async fn create_temp_file(...) -> Result<FsFileEntity>;
pub async fn delete_file(uid: i64, file_id: i64) -> Result<u64>;
pub async fn batch_delete_files(uid: i64, file_ids: Vec<i64>) -> Result<u64>;
pub async fn get_file_by_id(file_id: i64) -> Result<Option<FsFileEntity>>;
pub async fn get_file_by_object_key(object_key: &str) 
    -> Result<Option<FsFileEntity>>;
pub async fn batch_get_files(file_ids: Vec<i64>) 
    -> Result<Vec<FsFileEntity>>;
pub async fn list_user_files(uid: i64, limit: i64, offset: i64) 
    -> Result<Vec<FsFileEntity>>;
pub async fn list_app_files(app_id: &str, limit: i64, offset: i64) 
    -> Result<Vec<FsFileEntity>>;
pub async fn mark_files_as_official(...) -> Result<u64>;
pub async fn update_file_metadata(...) -> Result<FsFileEntity>;
pub async fn update_file_status(file_id: i64, status: i16) -> Result<u64>;
pub async fn stat_user_file_count(uid: i64) -> Result<i64>;
pub async fn stat_user_storage_used(uid: i64) -> Result<i64>;
```

### 修复步骤

**步骤 3.1**: 打开 `repository/src/cola_fs/pg/file.rs`

**步骤 3.2**: 在最后添加所有方法的 todo!() 版本:

```rust
impl FileRepo {
    // ... 现有 6 个方法 ...
    
    pub async fn delete_file(_uid: i64, _file_id: i64) -> Result<u64> {
        todo!("delete_file")
    }
    
    pub async fn batch_delete_files(_uid: i64, _file_ids: Vec<i64>) 
        -> Result<u64> {
        todo!("batch_delete_files")
    }
    
    // ... 继续添加其他方法 ...
}
```

**步骤 3.3**: 逐个实现方法

后续再用真实实现替换 todo!()

**验证**:
```bash
cargo build 2>&1 | grep -c "error\["
# 应该输出: 0
```

---

## 🔧 优先级 4: 补充依赖 (0.25 小时)

编辑 `cola_fs/Cargo.toml`:

```toml
[dependencies]
serde_json = "1.0.143"
chrono = { version = "0.4.42", features = ["serde"] }
```

---

## ✅ 最终验证

```bash
cargo build 2>&1
# 输出应该是:
# Finished dev [unoptimized + debuginfo] target(s) in XXs

cargo build 2>&1 | grep "^error"
# 应该输出空（0 个错误）
```

---

**完成后**: 编译通过，0 个错误 ✅
