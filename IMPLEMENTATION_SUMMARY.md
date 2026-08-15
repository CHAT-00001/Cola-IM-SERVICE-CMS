# 🎯 文件上传业务实现总结

**日期**: 2026/8/14  
**完成度**: 60%  
**预计完成时间**: 2小时

---

## ✅ 已完成

1. ✅ 所有 Port Trait 定义修正（14 个文件）
2. ✅ API 层完整实现（cola_fs/src/api/upload.rs）
3. ✅ CASE 层完整实现（cola_fs/src/case/upload.rs）
4. ✅ 编写了 Repository 实现指南

---

## ⏳ 立即需要完成

### 1. 修复 Repository 文件层

**文件**: `repository/src/cola_fs/pg/file.rs`

需要手动复制新的 13 个方法（因为编辑器大小限制）：
- create_temp_file() - 创建临时文件
- find_by_id() - 按 ID 查询
- find_by_object_key() - 按 key 查询
- batch_find_by_ids() - 批量查询
- list_user_files() - 用户文件列表
- list_app_files() - 应用文件列表
- mark_files_as_official() - **【关键】**标记为正式
- update_metadata() - 更新元数据
- update_status() - 更新状态
- delete_file() - 删除文件
- count_user_files() - 统计文件数
- sum_user_storage() - 统计存储容量
- cleanup_expired_orphan_files() - **【关键】**后台清理

详见：`FS_UPLOAD_IMPLEMENTATION.md`

### 2. 创建 Adapter 实现（关键路径）

**必须首先实现**：
- `repo_adapter/src/fs/file/manage.rs` - FileManageAdapter
  - `mark_files_as_official()` 【最关键】

**然后实现其他文件 Adapter**：
- add.rs、get.rs、check.rs、del.rs、list.rs、stat.rs

**最后实现媒体 Adapter**：
- media/add.rs、get.rs、check.rs、del.rs、list.rs、manage.rs、stat.rs

---

## 🚀 快速实现清单

```
[ ] 1. 修复 repository/src/cola_fs/pg/file.rs（手动复制）
[ ] 2. 创建 repo_adapter/src/fs/file/manage.rs（最重要）
[ ] 3. 创建 repo_adapter/src/fs/file/add.rs
[ ] 4. 创建 repo_adapter/src/fs/file/get.rs
[ ] 5. 创建 repo_adapter/src/fs/file/check.rs
[ ] 6. 创建 repo_adapter/src/fs/file/del.rs
[ ] 7. 创建 repo_adapter/src/fs/file/list.rs
[ ] 8. 创建 repo_adapter/src/fs/file/stat.rs
[ ] 9. 创建 repo_adapter/src/fs/media/所有实现
[ ] 10. 运行 cargo check 验证编译
[ ] 11. 编写单元测试
```

---

## 💡 Adapter 实现要点

每个 Adapter 遵循相同模式：

```rust
#[derive(Debug, Default, Clone)]
pub struct FileXxxAdapter;

#[async_trait]
impl FileXxxPort for FileXxxAdapter {
    async fn method_name(...) -> Result<T> {
        let pool = pg_pool();
        let result = FileRepo::xxx(&pool, ...).await?;
        tracing::info!("[🔌 ADAPTER] - ✅️ 操作成功");
        Ok(result)
    }
}
```

---

## 📞 关键 API 端点

```
POST /api/v2/fs/upload/key
  - 获取预签名 URL（app_id 区分业务桶）

POST /api/v2/fs/file/create
  - 创建临时文件记录

POST /api/v2/fs/file/mark-official
  - 【关键】标记文件为正式（UGC 发布后调用）

POST /api/v2/fs/media/create
  - 创建媒体资源（支持 LivePhoto）
```

---

## 🎬 核心业务流程

```
1. 客户端上传 → 获取预签名 URL
2. 上传文件到 S3
3. 创建临时文件记录 (status=0, expired_at=+7d)
4. UGC 发布 → 调用标记接口
5. 标记文件为正式 (status=1, expired_at=NULL)
6. 后台定时清理过期孤儿文件
```

---

**现在你可以按照清单继续实现！**

########## END
