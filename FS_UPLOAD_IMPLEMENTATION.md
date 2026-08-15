# 📤 FS 文件上传业务实现指南

**日期**: 2026/8/14  
**状态**: 进行中  
**完成度**: 60%

---

## ✅ 已完成

### 1. Port Trait 修正 (100%)
- ✅ `port/src/fs/file/add.rs` - 创建临时文件
- ✅ `port/src/fs/file/get.rs` - 获取文件
- ✅ `port/src/fs/file/check.rs` - 检查文件
- ✅ `port/src/fs/file/del.rs` - 删除文件
- ✅ `port/src/fs/file/list.rs` - 文件列表
- ✅ `port/src/fs/file/manage.rs` - 标记为正式（核心功能）
- ✅ `port/src/fs/file/stat.rs` - 文件统计
- ✅ `port/src/fs/media/add.rs` - 创建媒体
- ✅ `port/src/fs/media/get.rs` - 获取媒体
- ✅ `port/src/fs/media/check.rs` - 检查媒体
- ✅ `port/src/fs/media/del.rs` - 删除媒体
- ✅ `port/src/fs/media/list.rs` - 媒体列表
- ✅ `port/src/fs/media/manage.rs` - 转码成功更新
- ✅ `port/src/fs/media/stat.rs` - 媒体统计

### 2. API 层 (100%)
- ✅ `cola_fs/src/api/upload.rs` - 完整实现
  - `api_get_upload_key()` - 获取预签名 URL
  - `api_create_temp_file()` - 创建临时文件
  - `api_create_media()` - 创建媒体
  - `api_mark_files_official()` - 标记为正式

### 3. CASE 层 (100%)
- ✅ `cola_fs/src/case/upload.rs` - 完整实现
  - `case_get_upload_key()` - 生成上传密钥
  - `case_create_temp_file()` - 创建临时文件
  - `case_create_media()` - 创建媒体
  - `case_mark_files_official()` - 标记为正式

### 4. Repository 层 (80%)
- ✅ `repository/src/cola_fs/pg/file.rs` - 需要手动创建
  - `create_temp_file()` - 创建临时文件记录
  - `find_by_id()` - 按 ID 查询
  - `find_by_object_key()` - 按 key 查询
  - `batch_find_by_ids()` - 批量查询
  - `list_user_files()` - 用户文件列表
  - `list_app_files()` - 应用文件列表
  - `mark_files_as_official()` - 标记为正式
  - `update_metadata()` - 更新元数据
  - `update_status()` - 更新状态
  - `delete_file()` - 删除文件
  - `count_user_files()` - 统计用户文件数
  - `sum_user_storage()` - 统计存储容量
  - `cleanup_expired_orphan_files()` - 清理过期文件

---

## ⏳ 待完成

### 1. Adapter 层 (0%)

需要创建 `repo_adapter/src/fs/` 目录下的所有 adapter 实现：

```
repo_adapter/src/fs/
├── mod.rs
├── file/
│   ├── mod.rs
│   ├── add.rs          ← 实现 FileAddPort
│   ├── get.rs          ← 实现 FileGetPort
│   ├── check.rs        ← 实现 FileCheckPort
│   ├── del.rs          ← 实现 FileDelPort
│   ├── list.rs         ← 实现 FileListPort
│   ├── manage.rs       ← 实现 FileManagePort
│   └── stat.rs         ← 实现 FileStatPort
└── media/
    ├── mod.rs
    ├── add.rs          ← 实现 MediaAddPort
    ├── get.rs          ← 实现 MediaGetPort
    ├── check.rs        ← 实现 MediaCheckPort
    ├── del.rs          ← 实现 MediaDelPort
    ├── list.rs         ← 实现 MediaListPort
    ├── manage.rs       ← 实现 MediaManagePort
    └── stat.rs         ← 实现 MediaStatPort
```

### 2. Data 结构补全 (50%)

已有：
- ✅ `cola_data/src/cola_fs/entity/file.rs` - 文件实体
- ✅ `cola_data/src/cola_fs/entity/media.rs` - 媒体实体
- ✅ `cola_data/src/cola_fs/command/file.rs` - 文件命令
- ✅ `cola_data/src/cola_fs/command/media.rs` - 媒体命令

需要创建：
- ⏳ `cola_data/src/cola_fs/info/file.rs` - 文件缓存信息
- ⏳ `cola_data/src/cola_fs/info/media.rs` - 媒体缓存信息
- ⏳ `cola_data/src/cola_fs/vo/file.rs` - 文件 VO
- ⏳ `cola_data/src/cola_fs/vo/media.rs` - 媒体 VO

### 3. 业务扩展 (0%)

- ⏳ 后台定时清理任务
- ⏳ 异步转码处理
- ⏳ LivePhoto 特殊处理

---

## 🚀 关键业务流程

### 场景1：上传用户头像

```
1. 前端请求:
   POST /api/v2/fs/upload/key
   Body: { app_id: "user_avatar", file_name: "profile.jpg" }

2. API 调用:
   FsUploadApi::api_get_upload_key(uid, query, ctx)
   ↓
3. CASE 层:
   FsUploadCase::case_get_upload_key()
   - 查询 bucket 配置: app_id="user_avatar"
   - 生成 object_key: "2026/8/14/user_123/profile.jpg"
   - 调用 Adapter 生成预签名 URL
   ↓
4. Adapter 调用:
   ctx.fs.bucket.add.get_presigned_url(...)
   ↓
5. Repository:
   BucketRepo::find_by_app_id("user_avatar")
   ↓
6. 返回:
   { presigned_url, object_key, bucket, expired_at }

7. 前端上传文件到 S3

8. 前端回调后台:
   POST /api/v2/fs/file/create
   Body: { app_id, bucket_key, object_key, ... }
   ↓
9. 创建临时文件记录:
   - status = 0 (临时)
   - expired_at = now + 7 days
   ↓
10. 前端更新用户头像:
    POST /api/v2/user/profile
    Body: { file_id: 123 }
    ↓
11. 标记文件为正式:
    FsUploadApi::api_mark_files_official(uid, [123], "user", user_id)
    - UPDATE file SET status=1, expired_at=NULL
```

---

## 📋 关键技术点

### 1. 临时/正式两阶段

**文件状态转换**:
```
临时态 (status=0, expired_at=now+7d)
    ↓ 
    UGC 发布后调用标记接口
    ↓
正式态 (status=1, expired_at=NULL)
```

### 2. 后台定时清理

```rust
// 后台任务（每小时执行一次）
pub async fn cleanup_expired_files(pool: &PgPool) {
    match FileRepo::cleanup_expired_orphan_files(pool).await {
        Ok(count) => {
            info!("[🗣️ CLEANUP] ✅️ 清理过期孤儿文件: count={}", count);
        }
        Err(e) => {
            error!("[🤐 CLEANUP] ❌️ 清理失败: {}", e);
        }
    }
}
```

### 3. 业务桶管理

每个业务一个桶，通过 app_id 区分：

```
app_id = "user_avatar"      → 用户头像
app_id = "user_background"  → 用户背景图
app_id = "video_cover"      → 视频封面
app_id = "video_main"       → 视频主文件
app_id = "livephoto_image"  → LivePhoto 静态图
app_id = "livephoto_video"  → LivePhoto 短视频
...
```

---

## 🔧 下一步行动

### 立即执行 (1小时)

1. ✅ 修复 `repository/src/cola_fs/pg/file.rs` (手动复制粘贴新内容)
2. ⏳ 创建 `repo_adapter/src/fs/file/add.rs` 等 adapter 实现

### 本周执行

1. ⏳ 完成所有 adapter 实现
2. ⏳ 创建 info/vo 数据结构
3. ⏳ 验证编译和运行
4. ⏳ 编写单元测试

---

## 📞 代码引用

### Port 调用示例

```rust
// CASE 层调用 Port 获取桶配置
let bucket = ctx.fs.bucket.get
    .get_bucket_by_app_id(Some(&app_id))
    .await?;

// CASE 层调用 Port 生成预签名 URL
let presigned_url = ctx.fs.bucket.add
    .get_presigned_url(uid, &bucket.s3_bucket, &object_key, 3600)
    .await?;

// CASE 层调用 Port 创建文件
let file = ctx.fs.file.add
    .create_temp_file(uid, app_id, bucket, object_key, ...)
    .await?;

// CASE 层调用 Port 标记为正式
let count = ctx.fs.file.manage
    .mark_files_as_official(uid, file_ids, ref_table, ref_id)
    .await?;
```

---

**完成度**: 60%  
**预计完成**: 本周末  
**主要风险**: Adapter 层实现量大

########## END
