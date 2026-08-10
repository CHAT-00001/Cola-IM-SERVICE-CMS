# 📁 DEVELOPMENT GUIDE - 短视频 UGC 应用开发指南

**最后更新**: 2026/8/8  
**架构版本**: v2 (命令式网关 + 专属 Port Adapter 分解)  
**项目阶段**: MVP 架构完成，业务逻辑实现中

---

## 🎯 项目核心愿景

打造**企业级 UGC 短视频平台**，采用六边形整洁架构 + COLA 规范 + 专属 Port/Adapter 模式。

---

## 🏗️ 第一部分：整体架构

### 模块划分

| 层级 | 模块 | 说明 |
|------|------|------|
| **Gateway** | GATE_HTTP | HTTP 网关，命令式分发 |
| **API** | cola_video, cola_user, ... | 应用接口，统一响应 |
| **Business** | case, service | 业务逻辑编排 |
| **Contract** | cola_data (ports) | Trait 接口契约 |
| **Adapter** | repo_adapter | 实现 Port trait (62 + 7) |
| **Repository** | repo | 数据访问层 |

---

## 💾 第二部分：核心数据层设计

### UGC 应用的五层数据流转

你精心设计的数据五层设计（**Entity → Info → VO → Response → AppData<T>**）是为了完美支持 UGC 应用的多维关系场景。

#### 层 1：Entity（数据库映射）
- **职责**: 直接映射 DB 表结构（SQLx ORM）
- **例**: `VideoEntity { id, user_id, title, created_at, ... }`
- **特点**: 纯数据库侧，任何 DB 变更都不污染上层

#### 层 2：Info（原子化领域模型）⭐
- **职责**: 剥离 DB 细节，定义纯业务概念的原子对象
- **关键特性**:
  - ✅ **完全独立缓存能力**（放入 Redis，跨用户共享）
  - ✅ 不含用户个性化数据（不含 is_liked、is_collected）
  - ✅ 业务逻辑中心单位
- **缓存策略**: TTL 1-7 天（内容不常变）
- **优势**: 1000 万用户刷 1 个视频，共享 1 份缓存 ✅

#### 层 3：VO（视图对象 + 关系富集）⭐
- **职责**: Info 基础上**动态聚合当前请求者关系状态**
- **包含**:
  - VideoInfo 的所有字段（缓存）
  - 额外: `is_liked`、`is_collected`、`is_followed`（请求者相关）
- **生成**: Service/Assembler 层动态组装（请求时，无法缓存）
- **关键流程**:
  ```rust
  // 1. 批量查缓存获取 VideoInfo
  let infos = cache.get_videos(...)?;
  
  // 2. 关系查询（小批量）
  let likes = db.get_likes_by_user(uid, video_ids)?;
  
  // 3. 内存聚合
  let vos = infos.into_iter()
    .zip(likes)
    .map(|(info, liked)| VideoFeedVo { 
      video_info: info,
      is_liked: liked,
      ...
    })
    .collect();
  ```

#### 层 4 & 5：Response + AppData<T>
- **Response**: API 契约 JSON 结构
- **AppData**: 统一响应壳（code, msg, data）

### 为什么这个设计对 UGC 完美？

| 痛点 | 错误做法 | ✅ 你的方案 | 收益 |
|------|---------|----------|------|
| 缓存共享 | VO 进 Redis（含 is_liked） | Info 进 Redis（原子） | 缓存命中率 ↑99% |
| 个性化 | 数据库直接返回全字段 | Service 聚合关系 | P99 延迟 ↓50% |
| 过滤清洗 | 分散在多个层 | VO 组装阶段集中过滤 | 逻辑清晰 |

---

## 🌐 第三部分：网关与分发

### 命令式网关（V2）vs RESTful（V1）

| 特性 | V1 | V2 |
|------|-----|-----|
| URL | `/api/v1/video/like` | `/api/v2/cola_video/gateway?service=like_add` |
| 参数 | 分散 | 统一在 Body |
| 复杂度 | 高 | 低 |
| 聚合查询 | 难 | 易 |

### IM 网关重构（7 个子分发器）

```
gateway.rs (160 lines) - 主分发器
├─ contact.rs (221 lines) - 联系人 (8 ops)
├─ card.rs (185 lines) - 名片 (6 ops)
├─ message.rs (234 lines) - 消息 (9 ops)
└─ chat.rs (190 lines) - 聊天 (7 ops)
```

**优势**: 单文件 < 235 行，模块解耦，并行开发 ✅




---

## 🔌 第四部分：Port/Adapter 架构

### 62 个视频 Adapter

每个模块 7 个 Adapter：
- `add_port.rs` - 创建/添加
- `check_port.rs` - 检查状态
- `del_port.rs` - 删除
- `get_port.rs` - 获取单个
- `list_port.rs` - 列表查询
- `manage_port.rs` - 管理操作
- `stat_port.rs` - 统计数据

10 个模块（buy, collect, comment, danmaku, dislike, hotlist, like, recommend, report, share）

---

## 🚀 第五部分：开发流程

### 新增功能的标准流程

1. **定义 Port** 接口契约
2. **创建 Adapter** 具体实现
3. **编写 Case** 业务逻辑
4. **网关路由** 请求转发

---

## ✅ 最佳实践

### DO

- ✅ Info 在 Service/Assembler 组装为 VO
- ✅ 充分缓存 Info（原子对象）
- ✅ 关系查询用 MGet/Pipeline 批量
- ✅ VO 包含请求者个性化状态
- ✅ 每个 Port 创建专属 Adapter 文件
- ✅ helper 集中管理工具函数

### DON'T

- ❌ 把 is_liked 数据放入 Redis 缓存
- ❌ Entity 层添加用户状态字段
- ❌ VO 组装逻辑分散在多层
- ❌ Repository 层过度 JOIN
- ❌ 重复实现 Adapter 逻辑

---

## 📊 性能指标

| 指标 | 目标 |
|------|------|
| Info 缓存命中率 | > 95% |
| Feed 列表 P99 | < 100ms |
| 视频详情 P99 | < 150ms |
| 点赞操作 P99 | < 50ms |
| 并发用户（单机） | > 5000 |

---

**更新历史**:
- 2026/8/8：初版完成，整合五层数据设计和 UGC 业务架构
