# UPLOAD_PLAN_20260817

// D:\rust\short-video\UPLOAD_PLAN_20260817.md
// 🚧 FS - UGC 文件、媒体、LivePhoto 异步上传开发计划
// 2026/8/17 Created.

////////

## 1. 目标

建立面向 UGC 图文和短视频的统一上传链路：

```text
GATEWAY → API → CASE → PORT → ADAPTER → REPOSITORY
```

核心要求：

- 单次批量最多支持 20 个媒体对象；
- 客户端支持异步上传，文件直传对象存储；
- 一个媒体对象可以包含一个或多个物理文件；
- LivePhoto 作为一个媒体对象，关联静态图片和动态视频两个文件；
- 文件对象、媒体对象默认 7 天过期；
- UGC 未发布时，定期清理未绑定主内容的孤儿文件和媒体；
- UGC 发布成功后，将相关文件和媒体标记为正式资源；
- 旧版 `thumb`、`href`、`thumbnail`、`original_url` 保持现有旧版 CDN 组装逻辑；
- 新版 `media_ids` 通过 Port 查询媒体及其文件对象后组装；
- 公有桶使用 Bucket 的 `cdn_domain` 拼接 URL；
- 私有桶生成 S3 预签名 URL；
- 七牛云 S3 兼容接口使用 AWS Signature V4，S3 Endpoint 与 CDN Domain 分开处理。

////////

## 2. 核心数据模型

### 2.1 File：物理文件对象

对应现有：

```text
cola_data/src/cola_fs/entity/file.rs
```

File 记录一个真实上传对象：

```text
app_id
bucket
object_key
file_name
mime_type
file_size
file_hash
status
expired_at
created_at
updated_at
```

建议状态：

```text
0：临时文件/上传中
1：正式可用
2：上传失败
3：已删除
```

约束：

- `object_key` 必须服务端生成或校验；
- 文件必须绑定 `app_id` 和 Bucket；
- 文件必须记录 `expired_at = created_at + 7 天`；
- 前端不能提交或覆盖文件的所有者、状态和过期时间。

### 2.2 Media：业务媒体对象

对应现有：

```text
cola_data/src/cola_fs/entity/media.rs
```

一个 Media 是前端看到的一个图库项目，不等于一个物理文件：

```text
media_type = 1：普通图片
media_type = 2：普通视频
media_type = 3：LivePhoto
media_type = 4：HLS/多清晰度视频
```

现有文件引用字段：

```text
cover_file_id
main_file_id
aux_file_id
```

LivePhoto 约定：

```text
media_type = 3
main_file_id = 静态图片文件
aux_file_id  = 动态视频文件
cover_file_id = 可选封面文件
```

因此：

```text
10 个图库项目 = 10 个 Media
1 个 LivePhoto = 1 个 Media + 2 个 File
```

Media 也需要：

- `created_at`；
- `expired_at` 或等价过期字段；
- `status`；
- `app_id`；
- 创建者/所有者信息；
- 是否已经绑定主内容 UGC。

////////

## 3. 客户端异步上传流程

### 阶段 A：获取 Bucket 上传权限

业务 `app_id` 由客户端使用固定业务标识提交，例如：

```text
short-video
```

请求：

```http
POST /api/v2/fs/gateway?service=upload_key
Content-Type: application/json
```

请求体：

```json
{
  "service": "upload_key",
  "app_id": "short-video",
  "file_name": "photo.jpg"
}
```

服务端通过：

```text
ctx.fs.bucket.get.get_bucket_by_app_id(app_id)
```

获取安全的 `BucketInfo`，不向客户端返回：

```text
access_key
secret_key
config_json
```

响应至少包含：

```json
{
  "presigned_url": "https://...",
  "object_key": "2026/08/17/user_100/file.jpg",
  "bucket": "qiniu",
  "expired_at": "2026-08-17T...Z",
  "expires_in": 600
}
```

### 阶段 B：客户端异步直传

客户端可以并发上传最多 20 个媒体对象涉及的所有物理文件：

- 普通图片：1 个 File；
- 普通视频：1 个 File；
- LivePhoto：2 个 File；
- 并发数量由客户端控制，服务端限制单个批次最多 20 个 Media；
- 上传失败的文件可以单独重试；
- 不允许通过客户端重试重复创建正式媒体。

### 阶段 C：创建临时 File 记录

客户端上传成功后提交 File 创建命令：

```http
POST /api/v2/fs/gateway?service=file_create
```

```json
{
  "service": "file_create",
  "app_id": "short-video",
  "bucket": "qiniu",
  "object_key": "2026/08/17/user_100/photo.jpg",
  "original_name": "photo.jpg",
  "file_size": 102400,
  "mime_type": "image/jpeg",
  "file_hash": "sha256..."
}
```

服务端负责：

- 校验当前用户；
- 校验 Bucket 归属；
- 校验 Object Key；
- 校验对象是否真实存在或上传状态是否有效；
- 写入临时 File；
- 设置 7 天过期时间。

### 阶段 D：批量创建 Media

单次最多 20 个 Media：

```http
POST /api/v2/fs/gateway?service=media_batch_create
```

普通图片：

```json
{
  "service": "media_batch_create",
  "app_id": "short-video",
  "medias": [
    {
      "media_type": 1,
      "main_file_id": 10001,
      "cover_file_id": null,
      "aux_file_id": null
    }
  ]
}
```

LivePhoto：

```json
{
  "service": "media_batch_create",
  "app_id": "short-video",
  "medias": [
    {
      "media_type": 3,
      "main_file_id": 10002,
      "aux_file_id": 10003,
      "cover_file_id": null
    }
  ]
}
```

服务端必须保证：

- `medias.len() <= 20`；
- 普通图片/视频必须有 `main_file_id`；
- LivePhoto 必须有 `main_file_id` 和 `aux_file_id`；
- 所有 File 都属于当前用户、同一 `app_id` 和合法 Bucket；
- 所有 File 状态可用且未过期；
- 批量创建失败时使用事务，避免只创建半批 Media；
- Media 默认设置 7 天过期；
- 返回 20 个以内的 Media ID。

响应：

```json
{
  "list": [
    {
      "id": 20001,
      "media_type": 1,
      "main_file_id": 10001,
      "aux_file_id": null
    },
    {
      "id": 20002,
      "media_type": 3,
      "main_file_id": 10002,
      "aux_file_id": 10003
    }
  ],
  "count": 2
}
```

### 阶段 E：发布 UGC 主内容

主内容只提交 Media ID，不直接提交 File ID：

```json
{
  "title": "周末旅行",
  "description": "图文内容",
  "media_ids": [20001, 20002, 20003]
}
```

发布 CASE 负责：

- 校验 Media 所有权；
- 校验 Media 未过期；
- 校验 Media 状态可用；
- 校验 Media 数量和顺序；
- 写入 UGC 与 Media 的绑定关系；
- 将相关 File、Media 从临时态转正式态；
- 清空或延长过期时间；
- 发布失败不得留下无法追踪的正式对象。

////////

## 4. Media 查询与组装

### 4.1 新版 `media_ids`

视频/图文组装器不能把 `media_ids` 当作普通字符串直接输出。应由 CASE 通过 Port 查询：

```text
CASE
  → MediaGetPort::batch_get_medias(media_ids)
  → Adapter
  → Repository
  → MediaEntity
```

随后根据 Media 中的：

```text
cover_file_id
main_file_id
aux_file_id
```

批量查询 File 信息，并组装为安全的 MediaInfo：

```rust
MediaInfo {
    id,
    media_type,
    cover_file,
    main_file,
    aux_file,
}
```

不得让 Assembler 直接查询 Repository。

### 4.2 公开 Bucket

Bucket：

```text
is_public = true
```

资源地址：

```text
cdn_domain + object_key
```

需要处理：

- CDN 域名末尾 `/`；
- Object Key 开头 `/`；
- 已经是绝对 URL 的地址不重复拼接；
- Bucket 未配置 CDN 时使用固定兜底域名并记录 warning。

### 4.3 私有 Bucket

Bucket：

```text
is_public = false
```

不能直接返回：

```text
cdn_domain + object_key
```

需要通过 Bucket Port/Adapter 生成短期 S3 预签名 URL：

```rust
ctx.fs.bucket.add.get_presigned_url(
    uid,
    &bucket,
    &object_key,
    600,
)
.await?
```

建议有效期：

```text
600 秒
```

七牛云 S3 兼容接口：

- 使用 AWS Signature V4；
- `endpoint` 使用七牛 S3 API Endpoint；
- `region` 使用七牛实际区域；
- `bucket` 使用七牛真实 Bucket 名称；
- `cdn_domain` 不作为 S3 签名 Endpoint；
- 如果最终访问必须经过 CDN，需要另行实现七牛 CDN 下载鉴权，不可直接把 S3 签名参数拼到 CDN 域名上。

### 4.4 旧版字段兼容

保留旧版：

```text
thumb
href
thumbnail
original_url
```

旧版字段继续使用原有 CDN 组装逻辑，不因新版 Media 查询改动行为。

新版字段：

```text
media_ids
media
```

走新的：

```text
media_ids → Port → Media → File → Bucket → URL 策略
```

旧版和新版字段可以同时存在，但必须避免重复覆盖：

- 旧版字段由旧版逻辑处理；
- `media` 字段由 Media 组装逻辑处理；
- 公有/私有判断以对应 File 所属 Bucket 为准。

////////

## 5. Port / Adapter / Repository 任务

### 5.1 File Port

需要完善：

```text
create_temp_file
get_file_by_id
get_file_by_object_key
batch_get_files
mark_files_as_official
delete_expired_files
```

### 5.2 Media Port

需要完善：

```text
create_media
batch_create_medias
create_livephoto_media
get_media_by_id
batch_get_medias
batch_get_media_files
mark_medias_as_official
delete_expired_medias
```

### 5.3 Adapter

Adapter 负责：

- Entity → 安全 Info；
- Redis 缓存热点 Media/File；
- 批量查询去重；
- 私有 Bucket 预签名 URL；
- 屏蔽 `access_key` 和 `secret_key`；
- 记录每层日志。

### 5.4 Repository

Repository 负责：

- 批量插入 Media；
- 查询 Media 及 File 关联；
- 事务绑定 UGC 与 Media；
- 查询过期临时对象；
- 批量逻辑删除过期对象；
- 只返回数据库实体，不拼装业务 VO。

////////

## 6. 过期与孤儿清理

### 6.1 过期规则

临时 File：

```text
expired_at = created_at + 7 天
```

临时 Media：

```text
expired_at = created_at + 7 天
```

UGC 发布成功后：

- File 状态改为正式；
- Media 状态改为正式；
- 清除过期时间或改为长期生命周期；
- 写入 UGC 关联关系。

### 6.2 定期清理任务

建议每小时执行一次：

```text
1. 查询 expired_at < NOW() 的临时 Media；
2. 排除已经绑定正式 UGC 的 Media；
3. 查询这些 Media 关联的 File；
4. 删除或标记 Media；
5. 删除或标记 File；
6. 异步删除对象存储中的 Object；
7. 记录清理数量和失败对象。
```

必须支持幂等：

- 重复执行不会报错；
- 删除失败可重试；
- 数据库记录和对象存储删除状态可追踪；
- 不删除仍被正式 UGC 使用的文件。

### 6.3 孤儿判断

孤儿对象包括：

- 没有关联 Media 的 File；
- 没有关联正式 UGC 的临时 Media；
- Media 已过期且未发布；
- File 已过期且未被正式 Media 引用。

////////

## 7. 校验与安全

- 单批 Media 数量必须 `1..=20`；
- Media 内的 File 数量按媒体类型校验；
- File 必须属于当前用户或当前上传会话；
- `app_id` 必须与 Bucket 一致；
- `bucket` 必须与 Bucket 配置一致；
- 私有桶签名 URL 不得缓存为长期公开地址；
- `access_key`、`secret_key` 不得进入 API Response、Redis BucketInfo 或 MediaInfo；
- 所有临时资源默认 7 天过期；
- UGC 发布必须使用事务或可恢复状态机；
- 客户端重复提交要支持幂等键，避免重复创建 Media。

////////

## 8. 推荐实现顺序

### Phase 1：完善底层查询

1. 完成 File Repository 批量查询；
2. 完成 Media Repository 单条/批量查询；
3. 完成 Media Adapter 和 File Adapter；
4. 增加安全 `FileInfo`、`MediaInfo`。

### Phase 2：批量媒体创建

1. 增加 `BatchCreateMediaCmd`；
2. 增加 Gateway 分叉 `media_batch_create`；
3. 增加 API、CASE、Port、Adapter、Repository；
4. 限制单批最多 20 个 Media；
5. 完成 LivePhoto 两文件校验；
6. 增加事务和幂等处理。

### Phase 3：上传异步化

1. 完善 `upload_key` Gateway 分叉；
2. 服务端按 Bucket 生成预签名 URL；
3. 客户端并发直传；
4. 上传完成后创建 File；
5. 批量创建 Media；
6. 返回 Media ID 列表。

### Phase 4：Feed/UGC 组装

1. UGC CASE 接收 `media_ids`；
2. 通过 Port 批量查询 Media 和 File；
3. 根据 Bucket 公私有策略生成资源 URL；
4. 旧版字段保持旧 CDN 逻辑；
5. 新版 `media` 字段填充完整 MediaInfo；
6. LivePhoto 分别填充图片和视频 URL。

### Phase 5：清理与监控

1. 增加临时对象清理任务；
2. 增加对象存储删除重试；
3. 增加孤儿 File/Media 审计；
4. 统计上传成功率、失败率、过期清理量；
5. 监控预签名 URL 生成失败和 CDN 回源失败。

//////// END