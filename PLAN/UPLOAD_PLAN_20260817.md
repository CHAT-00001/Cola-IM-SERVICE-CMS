# UPLOAD_PLAN_20260817

// D:\rust\short-video\UPLOAD_PLAN_20260817.md
// 🚧 FS - 所有 UGC 通用的文件、媒体、LivePhoto 异步上传基础设施开发计划
// 2026/8/17 Created.
// 2026/8/17 Updated - 明确为跨 UGC 业务的统一上传协议与资源生命周期。

////////

## 1. 定位与目标

本计划建设的是 **FS 通用上传基础设施**，不是某一个短视频、图文或评论业务的私有上传接口。

所有需要用户上传媒体的 UGC 业务，都必须复用同一套 File / Media / UploadSession 能力，包括但不限于：

- 短视频、视频投稿；
- 图文、动态、帖子；
- 图片、相册；
- LivePhoto；
- 评论、回复、私信中的图片/视频；
- 用户资料中的头像、背景图；
- 后续扩展的音频、附件、HLS、多清晰度视频。

统一链路：

```text
UGC Gateway → FS Upload API → FS CASE → FS Port → FS Adapter → FS Repository / Object Storage
```

业务发布链路只负责自己的主内容和权限，不直接操作对象存储：

```text
UGC CASE
  → Upload/Media Port 校验并绑定 media_ids
  → 主内容 Repository 事务提交
  → FS 将 Media/File 转为正式资源
```

### 1.1 非目标

- FS 不负责创建视频、动态、评论等业务主表；
- FS 不依赖某个 UGC 表名或某个业务字段；
- 客户端不能直接指定资源所有者、状态、生命周期或对象存储凭证；
- UGC 业务不能把 File ID 当作对外资源 ID；
- Feed/Assembler 不得绕过 Port 直接查询 Repository。

### 1.2 通用核心要求

- 一个上传批次通过 `upload_session_id` 隔离，支持重试和幂等；
- 单批最多 20 个 Media，File 数量由 Media 类型决定；
- 一个 Media 可以关联一个或多个物理 File；
- LivePhoto 是 1 个 Media + 2 个必需 File，可选封面 File；
- 文件由客户端直传对象存储，服务端只发放短期上传凭证；
- 临时 File、Media 默认 7 天过期；
- 所有 UGC 业务通过通用绑定接口正式化资源；
- 未绑定正式主内容的资源可安全清理，清理必须幂等且可重试；
- 旧版 `thumb`、`href`、`thumbnail`、`original_url` 继续走旧 CDN 逻辑；
- 新版 `media_ids` 统一走 `Media → File → Bucket → URL` 策略；
- 公有桶使用 `cdn_domain`，私有桶使用短期 S3 预签名 URL；
- 七牛云使用 AWS Signature V4，S3 Endpoint 与 CDN Domain 严格分离。

////////

## 2. 通用领域模型

### 2.1 UploadSession：上传会话

上传会话是所有 UGC 上传的安全边界和幂等边界。建议字段：

```text
id / session_id
uid
app_id                  # 产品/应用隔离，例如 short-video
ugc_type                # 通用业务类型，例如 video、post、comment、avatar
ugc_draft_id            # 可选，业务草稿 ID，不等于正式主内容 ID
idempotency_key
status                  # 0进行中 1已完成 2已取消 3已过期
expired_at
created_at
updated_at
```

约束：

- `uid + app_id + idempotency_key` 唯一；
- `ugc_type` 只用于审计和策略，不让 FS 依赖具体业务表；
- 一个会话最多创建 20 个 Media；
- 会话取消后不能继续创建 Media，但允许幂等查询已有结果；
- 会话过期不会自动正式化资源。

### 2.2 File：物理文件对象

对应现有：

```text
cola_data/src/cola_fs/entity/file.rs
```

File 记录对象存储中的一个真实对象：

```text
id
session_id
uid
app_id
bucket_id / bucket
object_key
file_name
mime_type
file_size
file_hash
is_public
status
expired_at
created_at
updated_at
deleted_at
```

建议状态：

```text
0：临时/上传中
1：临时可用，等待 Media 或 UGC 绑定
2：上传失败/校验失败
3：正式可用
4：删除中
5：已删除
```

安全约束：

- `object_key` 必须由服务端生成，或使用服务端签发的 key 校验；
- File 必须绑定 `uid`、`session_id`、`app_id` 和 Bucket；
- `expired_at = created_at + 7 天`，客户端不可提交或覆盖；
- 创建 File 前必须校验对象存在、大小、MIME、Hash（能校验时）；
- File 正式化前必须确认仍属于当前用户和当前上传会话；
- `access_key`、`secret_key` 永远不能进入 FileInfo、API Response 或 Redis。

### 2.3 Media：业务媒体对象

Media 是 UGC 看到的一个媒体项目，不等于一个物理 File：

```text
id
session_id
uid
app_id
media_type
cover_file_id
main_file_id
aux_file_id
status
expired_at
bound_ref_type
bound_ref_id
sort
created_at
updated_at
deleted_at
```

媒体类型：

```text
1：普通图片       → main_file_id
2：普通视频       → main_file_id，可选 cover_file_id
3：LivePhoto      → main_file_id=静态图片，aux_file_id=动态视频
4：HLS/多清晰度   → main_file_id 或转码产物集合
5：音频           → main_file_id，可选封面
6：通用附件       → main_file_id
```

LivePhoto 约定：

```text
media_type = 3
main_file_id = 静态图片 File，必填
aux_file_id  = 动态视频 File，必填
cover_file_id = 可选封面 File
```

约束：

- Media 默认 7 天过期；
- Media 必须属于创建它的 `uid` 和 `upload_session_id`；
- Media 的 File 必须属于同一用户、应用和上传会话；
- 正式绑定后写入通用引用信息，不强制依赖某个 UGC 表；
- 一个 File 可以被多个 Media 引用时，清理器必须按引用计数保护它。

### 2.4 安全 Info / VO

对外只返回安全信息：

```rust
FileInfo {
    id,
    app_id,
    bucket,
    object_key,
    file_name,
    mime_type,
    file_size,
    status,
    url,
}

MediaInfo {
    id,
    media_type,
    cover_file,
    main_file,
    aux_file,
    sort,
}
```

`MediaInfo` 不得暴露存储供应商密钥、配置 JSON、内部供应商 ID 或数据库敏感字段。

////////

## 3. 通用 API 契约

所有 UGC 业务复用 `/api/v2/fs/gateway`，Body 为主，URL 参数只作为兼容补充。Gateway 统一使用 `ApiGatewayRequest`，不得为某个业务自建解析结构体。

### 3.1 创建上传会话

```http
POST /api/v2/fs/gateway?service=upload_session_create
```

```json
{
  "service": "upload_session_create",
  "app_id": "short-video",
  "ugc_type": "post",
  "idempotency_key": "client-generated-uuid",
  "ugc_draft_id": 123
}
```

响应至少包含：

```json
{
  "session_id": "us_xxx",
  "expires_at": "2026-08-24T00:00:00Z",
  "max_medias": 20
}
```

### 3.2 获取对象存储上传凭证

```http
POST /api/v2/fs/gateway?service=upload_key
```

```json
{
  "service": "upload_key",
  "session_id": "us_xxx",
  "file_name": "photo.jpg",
  "mime_type": "image/jpeg",
  "file_size": 102400,
  "file_hash": "sha256..."
}
```

响应至少包含：

```json
{
  "upload_url": "https://...",
  "object_key": "2026/08/17/us_xxx/file.jpg",
  "bucket": "qiniu",
  "expires_in": 600,
  "expired_at": "2026-08-17T00:10:00Z"
}
```

不得返回 `access_key`、`secret_key`、`config_json`。上传 URL 有效期固定为短期，建议 600 秒。

### 3.3 创建临时 File

```http
POST /api/v2/fs/gateway?service=file_create
```

客户端只提交 `session_id`、上传凭证返回的 `object_key` 和文件元数据；服务端自行确定 `uid`、`app_id`、Bucket、状态和过期时间。

服务端必须校验：

- 会话属于当前用户且未过期；
- Object Key 属于当前会话；
- Bucket 与 `app_id` 配置一致；
- 对象真实存在且大小、MIME、Hash 符合；
- 重试时使用幂等键，不重复创建 File。

### 3.4 批量创建 Media

```http
POST /api/v2/fs/gateway?service=media_batch_create
```

```json
{
  "service": "media_batch_create",
  "session_id": "us_xxx",
  "idempotency_key": "media-request-uuid",
  "medias": [
    {
      "media_type": 1,
      "main_file_id": 10001,
      "cover_file_id": null,
      "aux_file_id": null,
      "sort": 1
    },
    {
      "media_type": 3,
      "main_file_id": 10002,
      "aux_file_id": 10003,
      "cover_file_id": null,
      "sort": 2
    }
  ]
}
```

必须保证：

- `medias.len()` 在 `1..=20`；
- 普通图片、视频、音频、附件必须有 `main_file_id`；
- LivePhoto 必须有 `main_file_id` 和 `aux_file_id`；
- 所有 File 属于当前用户、会话、`app_id` 和合法 Bucket；
- File 状态为可用且未过期；
- 校验失败时整批回滚，不产生半批 Media；
- 相同幂等键重复提交返回第一次结果；
- 返回按 `sort` 排序的 Media ID 和 MediaInfo。

### 3.5 查询和取消上传

必须提供：

```text
upload_session_get
upload_session_cancel
media_batch_get
```

客户端重试、页面恢复和断网恢复优先查询会话状态，不重新创建正式 Media。

### 3.6 通用 UGC 绑定/正式化

UGC 业务发布成功时调用通用绑定能力，主内容只提交 Media ID：

```json
{
  "session_id": "us_xxx",
  "media_ids": [20001, 20002],
  "ref_type": "video",
  "ref_id": 30001,
  "idempotency_key": "publish-request-uuid"
}
```

绑定 Port 必须完成：

1. 校验 Media 所有权、状态、过期时间和顺序；
2. 校验 `ref_type/ref_id` 的业务授权由 UGC CASE 完成；
3. 在可恢复事务中写入引用关系；
4. 将 Media 和其未被其他正式 Media 引用的 File 标记为正式；
5. 清除或延长过期时间；
6. 重试返回相同绑定结果，不重复绑定。

FS 不直接创建 UGC 主表，但必须提供可被 UGC 事务调用的 `bind_medias` / `finalize_resources` Port。业务无法使用跨库事务时，使用绑定状态机和补偿任务。

////////

## 4. 资源 URL 与旧版兼容

### 4.1 新版 Media 组装

```text
media_ids
  → UGC CASE
  → MediaGetPort::batch_get_medias
  → FileGetPort::batch_get_files
  → Bucket Port
  → URL Strategy
  → MediaInfo
```

Assembler 只能消费安全 Info，不得直接访问 Repository。

### 4.2 公有 Bucket

当 `is_public = true` 时：

```text
url = normalize(cdn_domain) + "/" + normalize(object_key)
```

必须处理：

- CDN 域名末尾 `/`；
- Object Key 开头 `/`；
- 已是绝对 URL 时不得重复拼接；
- 没有 CDN 域名时使用明确配置的兜底域名并记录 warning；
- CDN Domain 只能用于下载地址，不能用于 S3 签名 Endpoint。

### 4.3 私有 Bucket

私有资源必须通过 Bucket Port/Adapter 生成短期下载 URL：

```rust
get_presigned_url(uid, bucket, object_key, 600).await?
```

七牛云 S3 兼容接口必须：

- 使用 AWS Signature V4；
- 使用七牛 S3 API Endpoint；
- 使用实际 region；
- 使用真实 Bucket 名称；
- 不把 CDN Domain 当作签名 Endpoint；
- 不把 S3 签名参数直接拼到 CDN Domain。

### 4.4 旧版字段

以下字段继续使用原有 CDN 组装逻辑：

```text
thumb / href / thumbnail / original_url
```

新版字段独立处理：

```text
media_ids / media
```

两套逻辑可以同时存在，但不得互相覆盖。所有 UGC 新接口优先使用 `media_ids`，旧字段只用于兼容历史数据。

////////

## 5. 分层职责与接口任务

### 5.1 GATEWAY / DISPATCHER

- Gateway 解析 URL + Body，Body 字段覆盖 URL 字段；
- 完成会话身份校验和 `uid` 注入；
- 按 `service` 分发到 FS API；
- Dispatcher 只路由，不做业务校验；
- 支持 `upload_session_create`、`upload_key`、`file_create`、`media_batch_create`、`media_batch_get`、`upload_session_cancel`。

### 5.2 API / CASE

API 负责权限编排和 `AppData<T>` 响应壳；CASE 负责：

- 会话、用户和应用校验；
- File / Media 类型校验；
- 批量和幂等编排；
- 调用 Port 完成绑定和正式化；
- 使用 `[🗣️ CASE]` / `[🤐 CASE]` 日志。

### 5.3 File Port

至少实现：

```text
create_temp_file
get_file_by_id
get_file_by_object_key
batch_get_files
mark_files_as_official
delete_expired_files
delete_orphan_files
```

### 5.4 Media Port

至少实现：

```text
create_media
batch_create_medias
create_livephoto_media
get_media_by_id
batch_get_medias
batch_get_media_files
mark_medias_as_official
delete_expired_medias
delete_orphan_medias
bind_medias
```

### 5.5 UploadSession / Binding Port

至少实现：

```text
create_session
get_session
cancel_session
check_idempotency
bind_medias
finalize_resources
```

### 5.6 Adapter

- 调 Repository 并转换 Entity → 安全 Info；
- 批量查询去重，保持请求顺序；
- 可缓存热点 File/Media，但缓存不得包含密钥；
- 生成公有 CDN URL 和私有预签名 URL；
- 处理对象存储删除、重试和错误映射；
- 使用 `[🔌 ADAPTER]` 日志。

### 5.7 Repository

- 只返回数据库 Entity，不拼装业务 VO；
- 支持批量插入、查询、绑定和状态更新；
- 支持按会话、用户、过期时间查询；
- 支持幂等键唯一约束；
- 支持事务或可恢复状态机；
- 支持逻辑删除和对象删除任务记录。

////////

## 6. 生命周期、清理与失败恢复

### 6.1 生命周期

```text
UploadSession: 进行中 → 已完成 / 已取消 / 已过期
File: 临时 → 可用 → 正式 → 删除中 → 已删除
Media: 临时 → 可用 → 正式 → 删除中 → 已删除
```

UGC 发布成功后：

- 写入 `ref_type/ref_id`；
- Media、相关 File 转正式；
- 清空或延长过期时间；
- 保留审计信息。

UGC 发布失败时：

- 不得留下不可追踪的正式资源；
- 保留临时状态供重试或由清理器回收；
- 绑定过程必须可重试、可补偿、可审计。

### 6.2 定期清理

建议每小时执行，且使用分布式锁：

1. 查询过期且非正式的 Session、Media、File；
2. 排除仍被正式 Media 或正式 UGC 引用的 File；
3. 标记数据库记录为删除中；
4. 异步删除对象存储 Object；
5. 成功后标记已删除，失败写入重试队列；
6. 记录清理数量、失败数量和最后错误。

孤儿对象包括：

- 无 Media 引用的临时 File；
- 无正式 UGC 绑定的过期 Media；
- 已取消/过期 Session 下的资源；
- File 已过期且无任何正式引用。

要求：重复执行幂等、删除失败可重试、对象和数据库状态可追踪。

////////

## 7. 通用安全、配额与可观测性

- 单批 Media 数量必须 `1..=20`；
- 限制单 Session 的总 File 数量、总字节数和并发凭证数量；
- 文件类型、扩展名、大小、Hash 和媒体结构必须服务端校验；
- File 必须属于当前用户和当前 Session；
- `app_id`、Bucket、Session 必须一致；
- 不能信任客户端提交的 `uid`、status、expired_at、owner、bucket_id；
- 私有 URL 不得作为长期公开地址缓存；
- API、Redis、日志中禁止打印密钥和完整签名；
- 幂等键必须有唯一约束和过期策略；
- UGC 业务绑定必须校验主内容权限；
- 所有层按规范记录成功/失败日志；
- 统计上传成功率、失败率、平均耗时、过期清理量、删除失败量；
- 监控预签名失败、对象不存在、Hash 不匹配、CDN 回源失败和重复提交。

////////

## 8. 各 UGC 业务接入规范

新增 UGC 业务不得复制上传 API，只需：

1. 定义自己的 `ugc_type`；
2. 创建 `upload_session`，携带业务草稿 ID（可选）；
3. 复用 `upload_key`、`file_create`、`media_batch_create`；
4. 主内容只接收 `media_ids`，不接收 File ID；
5. 在自己的 UGC CASE 中校验主内容权限、数量和顺序；
6. 调用 `bind_medias/finalize_resources`；
7. 输出时调用通用 Media Assembler；
8. 为自己的 `ugc_type` 增加权限、配额、回滚和集成测试。

业务差异只能通过策略配置或 Port 扩展表达，不能在 FS 中堆叠 `if video`、`if comment` 等业务分支。

////////

## 9. 实施阶段与验收标准

### Phase 0：契约和数据库设计

1. 确认 UploadSession、File、Media、Binding、DeleteTask 表结构；
2. 增加 `session_id`、`uid`、`app_id`、生命周期和引用字段；
3. 增加幂等键唯一索引；
4. 定义通用 `FileInfo`、`MediaInfo`、请求命令和错误码；
5. 明确跨库事务或状态机方案。

验收：迁移可执行，敏感字段不会进入 Info/Response，重复幂等键行为明确。

### Phase 1：底层 Repository / Port / Adapter

1. 完成 File / Media / Session Repository；
2. 完成单条、批量查询和去重；
3. 完成安全 Info 转换；
4. 完成正式化、绑定、逻辑删除；
5. 完成 Redis 缓存策略和失效策略。

验收：不含 `todo!`，单元测试覆盖空数组、重复 ID、越权和不存在资源。

### Phase 2：通用 Gateway 和异步上传

1. 接入所有通用 service 分叉；
2. 实现 UploadSession；
3. 实现按 Bucket 的短期上传凭证；
4. 实现真实 AWS Signature V4；
5. 实现 File 创建和对象存在性校验；
6. 支持客户端并发直传、单文件重试和会话恢复。

验收：任何 UGC 只通过通用接口上传，客户端拿不到密钥，上传凭证有效期不超过 600 秒。

### Phase 3：批量 Media 和复合媒体

1. 增加 `BatchCreateMediaCmd`；
2. 实现 20 个 Media 限制；
3. 实现图片、视频、音频、附件校验；
4. 实现 LivePhoto 两 File 校验；
5. 实现事务、幂等和按序返回；
6. 返回 Media ID 列表和安全 MediaInfo。

验收：普通媒体、LivePhoto、空批次、21 个媒体、跨用户 File、重复请求全部有自动化测试。

### Phase 4：UGC 绑定和资源组装

1. 为视频、图文、评论等第一个 UGC 业务接入 `media_ids`；
2. 实现通用绑定和正式化；
3. 实现 Media/File/ Bucket URL 组装；
4. 保持旧版字段 CDN 行为不变；
5. LivePhoto 分别填充图片和视频 URL；
6. 验证公有、私有和七牛 Bucket。

验收：发布成功资源永久可用，发布失败资源可恢复或可清理，旧版接口回归测试通过。

### Phase 5：清理、重试和监控

1. 增加每小时过期清理；
2. 增加对象删除任务和指数退避重试；
3. 增加孤儿 File/Media 审计；
4. 增加上传和清理指标；
5. 增加告警和后台审计查询；
6. 增加故障恢复演练。

验收：清理任务可重复执行，不删除正式 UGC 使用的对象，删除失败可追踪并自动重试。

### Phase 6：多业务推广

按以下顺序接入，验证接口确实通用：

1. 短视频/视频投稿；
2. 图文/动态；
3. 评论和回复；
4. 头像、背景图等用户内容；
5. 音频、附件和后续业务。

每个业务必须提供接入清单、权限测试、幂等测试、发布失败回滚测试和旧版兼容测试。

//////// END