# 快速开始指南 - Port Adapter 开发

## 🎯 5分钟快速上手

### 1️⃣ 生成所有 Adapter 框架（一行命令）

```bash
python gen_adapters_simple.py
```

生成 52 个待完成的 adapter 文件框架。

### 2️⃣ 选择一个 adapter 并打开

示例：开发 `like` 模块的 `add` adapter

```
打开: repo_adapter/src/video/like/add_port.rs
```

### 3️⃣ 按照模板填充

```rust
// 第1步：添加正确的导入
use cola_data::cola_video::port::like::add::LikeAddPort;

// 第2步：实现 trait
#[async_trait]
impl LikeAddPort for LikeAddPortAdapter {
    async fn save_like_record(&self, uid: i64, video_id: i64) -> Result<i64> {
        // 实现具体逻辑：
        // 1. 验证用户ID和视频ID是否存在
        // 2. 检查用户是否已点赞过该视频
        // 3. 保存点赞记录到数据库
        // 4. 返回点赞ID或错误
        
        // 示例伪代码：
        // let existing = check_existing_like(uid, video_id).await?;
        // if existing {
        //     return Err(anyhow!("Already liked"));
        // }
        // let like_id = save_to_db(uid, video_id).await?;
        // Ok(like_id)
        
        todo!()  // 先用 todo!() 占位，后续补全
    }

    async fn del_like_record(&self, uid: i64, video_id: i64) -> Result<()> {
        // 删除点赞记录
        todo!()
    }
}
```

### 4️⃣ 在 mod.rs 中声明

编辑 `repo_adapter/src/video/like/mod.rs`：

```rust
pub mod add_port;  // 新增此行
```

### 5️⃣ 在 lib.rs 中更新初始化

编辑 `repo_adapter/src/lib.rs` 的 `build_app_context()` 函数：

```rust
like: LikePort {
    add: Arc::new(video::like::add_port::LikeAddPortAdapter),  // 改这里
    // ... 其他字段
}
```

## 📋 所有需要完成的 Adapter

### Collect (收藏)
- collect/del_port.rs
- collect/get_port.rs
- collect/list_port.rs
- collect/manage_port.rs
- collect/stat_port.rs

### Comment (评论)
- comment/add_port.rs
- comment/check_port.rs
- comment/del_port.rs
- comment/get_port.rs
- comment/list_port.rs
- comment/manage_port.rs
- comment/stat_port.rs

### Danmaku (弹幕)
- danmaku/add_port.rs
- danmaku/check_port.rs
- danmaku/del_port.rs
- danmaku/get_port.rs
- danmaku/list_port.rs
- danmaku/manage_port.rs
- danmaku/stat_port.rs

### Dislike (不喜欢)
- dislike/add_port.rs
- dislike/del_port.rs
- dislike/list_port.rs
- dislike/manage_port.rs
- dislike/stat_port.rs

### Hotlist (热门)
- hotlist/add_port.rs
- hotlist/check_port.rs
- hotlist/del_port.rs
- hotlist/get_port.rs
- hotlist/list_port.rs
- hotlist/manage_port.rs
- hotlist/stat_port.rs

### Like (点赞)
- like/add_port.rs
- like/check_port.rs
- like/del_port.rs
- like/get_port.rs
- like/list_port.rs
- like/manage_port.rs
- like/stat_port.rs

### Recommend (推荐)
- recommend/add_port.rs
- recommend/check_port.rs
- recommend/del_port.rs
- recommend/get_port.rs
- recommend/list_port.rs
- recommend/manage_port.rs
- recommend/stat_port.rs

### Report (举报)
- report/add_port.rs
- report/check_port.rs
- report/del_port.rs
- report/get_port.rs
- report/list_port.rs
- report/manage_port.rs
- report/stat_port.rs

### Share (分享)
- share/add_port.rs
- share/check_port.rs
- share/del_port.rs
- share/get_port.rs
- share/list_port.rs
- share/manage_port.rs
- share/stat_port.rs

## 🎓 学习现有实现

### 参考已完成的 Buy 模块

所有 Buy 模块的 adapter 都已完成，可作为参考：

- `repo_adapter/src/video/buy/add_port.rs` - 完整实现示例
- `repo_adapter/src/video/buy/check_port.rs` - 简单 trait 实现
- `repo_adapter/src/video/buy/stat_port.rs` - 统计相关实现

## 💡 开发技巧

### 使用 todo!() 占位符

开发时可先用 `todo!()` 占位，让代码能编译，然后逐步补全：

```rust
#[async_trait]
impl SomePort for SomeAdapter {
    async fn method1(&self) -> Result<Type> {
        todo!("实现用户验证逻辑")
    }
    
    async fn method2(&self) -> Result<Type> {
        // 已实现的方法
        Ok(some_value)
    }
}
```

### 调试技巧

在实现时可以使用日志输出来调试：

```rust
use tracing::{info, error};

#[async_trait]
impl SomePort for SomeAdapter {
    async fn save_record(&self, id: i64) -> Result<()> {
        info!("[🗣️ ADAPTER]: 开始保存记录 id={}", id);
        
        // 业务逻辑...
        
        info!("[🗣️ ADAPTER]: ✅️ 保存成功");
        Ok(())
    }
}
```

## 🔗 关键文档

- 📖 `ADAPTER_DEVELOPMENT_GUIDE.md` - 完整开发指南
- 📊 `PROJECT_STATUS.md` - 项目状态总结
- 🎯 `AGENTS.md` - 编码规范（重要！）
- ✅ `PORTS_TEMPLATE.md` - 所有需创建的清单

## ⚡ 常见问题

### Q: 如何知道某个 trait 有哪些方法？
A: 查看 `cola_data/src/cola_video/port/{module}/{action}.rs` 文件中的 trait 定义

### Q: 可以用 todo!() 提交吗？
A: 可以！这样至少项目能编译。后续再逐个补全实现。

### Q: 多人开发会冲突吗？
A: 不会！因为每个文件职责独立。不同人可同时开发不同模块。

### Q: 如何运行测试？
A: 为每个 adapter 在 `tests/` 目录下创建对应的测试文件即可。

## 📞 联系方式

如有问题，请参考：
- AGENTS.md 中的规范要求
- ADAPTER_DEVELOPMENT_GUIDE.md 中的详细说明
- 已完成的 Buy 模块 adapter 作为参考实现

---

**祝你开发愉快！** 🚀

