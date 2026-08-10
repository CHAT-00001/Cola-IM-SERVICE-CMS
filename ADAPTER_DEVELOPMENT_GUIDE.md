# Port Adapter 开发指南

## 架构概述

项目采用 **一个 trait 对应一个物理文件** 的架构模式。这样的好处：

- ✅ 文件职责清晰单一
- ✅ 便于多人协作（不同人可并行开发不同 adapter）
- ✅ 便于单元测试（每个 adapter 独立测试）
- ✅ 便于维护扩展（修改某个功能只需改对应文件）
- ✅ 代码审查范围明确

## 文件结构

```
repo_adapter/src/video/
├── buy/                   # 购买模块
│   ├── mod.rs            # 模块声明
│   ├── add_port.rs       # ✅ 已完成：BuyAddPort 实现
│   ├── check_port.rs     # ✅ 已完成：BuyCheckPort 实现
│   ├── del_port.rs       # ✅ 已完成：BuyDelPort 实现
│   ├── get_port.rs       # ✅ 已完成：BuyGetPort 实现
│   ├── list_port.rs      # ✅ 已完成：BuyListPort 实现
│   ├── manage_port.rs    # ✅ 已完成：BuyManagePort 实现
│   └── stat_port.rs      # ✅ 已完成：BuyStatPort 实现
│
├── collect/              # 收藏模块
│   ├── mod.rs           # 模块声明
│   ├── add_port.rs      # ✅ 已完成：CollectAddPort 实现
│   ├── check_port.rs    # ✅ 已完成：CollectCheckPort 实现
│   ├── del_port.rs      # ⏳ 需完成
│   ├── get_port.rs      # ⏳ 需完成
│   ├── list_port.rs     # ⏳ 需完成
│   ├── manage_port.rs   # ⏳ 需完成
│   └── stat_port.rs     # ⏳ 需完成
│
├── comment/             # 评论模块 (需完成)
├── danmaku/             # 弹幕模块 (需完成)
├── dislike/             # 不喜欢模块 (需完成)
├── hotlist/             # 热门模块 (需完成)
├── like/                # 点赞模块 (需完成)
├── recommend/           # 推荐模块 (需完成)
├── report/              # 举报模块 (需完成)
└── share/               # 分享模块 (需完成)
```

## 创建新 Port Adapter 的步骤

### 步骤 1: 创建文件

**位置:** `repo_adapter/src/video/{module}/{action}_port.rs`

**示例:** `repo_adapter/src/video/collect/del_port.rs`

### 步骤 2: 实现 Trait

使用以下模板：

```rust
// repo_adapter/src/video/{module}/{action}_port.rs  -- {module} {action} Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::{module}::{action}::*;

////////

/// # [ADAPTER] - {module} {action} 
/// * `desc`: {功能描述}
#[derive(Debug, Default, Clone)]
pub struct {Module}{Action}PortAdapter;

#[async_trait]
impl {TraitName} for {Module}{Action}PortAdapter {
    async fn method_name(&self, param: Type) -> Result<ReturnType> {
        // TODO: 实现具体的数据库操作逻辑
        // 1. 参数验证
        // 2. 数据库查询/更新
        // 3. 结果处理
        todo!()
    }
}

//////// END
```

### 步骤 3: 添加模块声明

在 `repo_adapter/src/video/{module}/mod.rs` 中添加：

```rust
pub mod {action}_port;
```

### 步骤 4: 更新 lib.rs

在 `repo_adapter/src/lib.rs` 的 `build_app_context()` 函数中，将初始化改为指向新创建的 adapter：

```rust
// 从
{module}: {Module}{Action}Port {
    {action}: Arc::new(stub::GeneralStubAdapter),
    ...
}

// 改为
{module}: {Module}{Action}Port {
    {action}: Arc::new(video::{module}::{action}_port::{Module}{Action}PortAdapter),
    ...
}
```

## 全部需要完成的 Adapter 列表

### Collect (收藏) - 需完成 5 个
- [ ] del_port.rs -> CollectDelPort
- [ ] get_port.rs -> CollectGetPort
- [ ] list_port.rs -> CollectListPort
- [ ] manage_port.rs -> CollectManagePort
- [ ] stat_port.rs -> CollectStatPort

### Comment (评论) - 需完成 7 个
- [ ] add_port.rs -> AddPort
- [ ] check_port.rs -> VideoCommentCheckPort
- [ ] del_port.rs -> VideoCommentDelPort
- [ ] get_port.rs -> VideoCommentGetPort
- [ ] list_port.rs -> VideoCommentListPort
- [ ] manage_port.rs -> VideoCommentManagePort
- [ ] stat_port.rs -> VideoCommentStatPort

### Danmaku (弹幕) - 需完成 7 个
- [ ] add_port.rs -> DanmakuAddPort
- [ ] check_port.rs -> DanmakuCheckPort
- [ ] del_port.rs -> DanmakuDelPort
- [ ] get_port.rs -> DanmakuGetPort
- [ ] list_port.rs -> DanmakuListPort
- [ ] manage_port.rs -> DanmakuManagePort
- [ ] stat_port.rs -> DanmakuStatPort

### Dislike (不喜欢) - 需完成 5 个
- [ ] add_port.rs -> DislikeAddPort
- [ ] del_port.rs -> DislikeDelPort
- [ ] list_port.rs -> DislikeListPort
- [ ] manage_port.rs -> DislikeManagePort
- [ ] stat_port.rs -> VideoDislikeStatPort

### Hotlist (热门) - 需完成 7 个
- [ ] add_port.rs -> HotlistAddPort
- [ ] check_port.rs -> VideoHotlistCheckPort
- [ ] del_port.rs -> VideoHotlistDelPort
- [ ] get_port.rs -> VideoHotlistGetPort
- [ ] list_port.rs -> VideoHotlistListPort
- [ ] manage_port.rs -> VideoHotlistManagePort
- [ ] stat_port.rs -> VideoHotlistStatPort

### Like (点赞) - 需完成 7 个
- [ ] add_port.rs -> LikeAddPort
- [ ] check_port.rs -> LikeCheckPort
- [ ] del_port.rs -> LikeDelPort
- [ ] get_port.rs -> LikeGetPort
- [ ] list_port.rs -> LikeListPort
- [ ] manage_port.rs -> LikeManagePort
- [ ] stat_port.rs -> LikeStatPort

### Recommend (推荐) - 需完成 7 个
- [ ] add_port.rs -> AddPort
- [ ] check_port.rs -> VideoRecommendCheckPort
- [ ] del_port.rs -> VideoRecommendDelPort
- [ ] get_port.rs -> VideoRecommendGetPort
- [ ] list_port.rs -> VideoRecommendListPort
- [ ] manage_port.rs -> VideoRecommendManagePort
- [ ] stat_port.rs -> VdieoRecommendStatPort

### Report (举报) - 需完成 7 个
- [ ] add_port.rs -> VideoReportAddPort
- [ ] check_port.rs -> VideoReportCheckPort
- [ ] del_port.rs -> VideoReportDelPort
- [ ] get_port.rs -> VideoReportGetPort
- [ ] list_port.rs -> VideoReportListPort
- [ ] manage_port.rs -> ReportManagePort
- [ ] stat_port.rs -> ReportStatPort

### Share (分享) - 需完成 7 个
- [ ] add_port.rs -> VideoShareAddPort
- [ ] check_port.rs -> VideoShareCheckPort
- [ ] del_port.rs -> VideoShareDelPort
- [ ] get_port.rs -> VideoShareGetPort
- [ ] list_port.rs -> VideoShareListPort
- [ ] manage_port.rs -> VideoShareManagePort
- [ ] stat_port.rs -> VideoShareStatPort

## 总计

- **已完成:** 9 个文件
- **需完成:** 53 个文件
- **总计:** 62 个 Port Adapter 文件

## 开发建议

1. **优先级排序**
   - P1: buy、collect、like、comment（最常用的功能）
   - P2: danmaku、share、report（社交功能）
   - P3: hotlist、recommend、dislike（运营功能）

2. **并行开发**
   - 不同的人可以同时开发不同模块的 adapter
   - 因为每个文件的职责清晰，不会互相影响

3. **测试建议**
   - 为每个 adapter 编写单元测试
   - 测试位置：`repo_adapter/src/video/{module}/tests/{action}_port_test.rs`

4. **代码审查**
   - 每个 adapter 文件单独审查
   - 关注点：参数验证、错误处理、事务管理

## 现状说明

**当前采用的过渡方案：**

在完成全部 62 个 adapter 文件之前，项目使用一个临时的 `TempVideoPortAdapter` 来满足编译需求。这个临时 adapter 返回固定的默认值（Ok(())、Ok(0)、Ok(vec![])等）。

**后续工作流程：**

1. 根据本指南逐个创建专属的 adapter 文件
2. 实现对应的业务逻辑
3. 在 lib.rs 中逐个替换临时 adapter
4. 最终移除 `TempVideoPortAdapter`

这样做的优势：
- ✅ 项目可以立即编译运行
- ✅ 保留了专属 adapter 的架构优势
- ✅ 开发工作可以逐步推进，不会一次性阻塞整个项目
- ✅ 代码审查和测试可以逐个进行

