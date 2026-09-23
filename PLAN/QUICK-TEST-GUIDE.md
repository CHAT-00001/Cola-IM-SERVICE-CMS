# 🧪 快速测试指南 - 权限检查

**日期**: 2026-09-19  
**版本**: 1.0

---

## 📌 测试前准备

### 确保系统启动
```bash
cd d:\rust\short-video
cargo run --release
```

### 访问网关
```
http://127.0.0.1:8080/api/v2/video/gateway
```

---

## 🔍 测试场景

### ✅ 测试 1: 游客获取弹幕（权限=1，允许）

**请求**:
```bash
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=get_danmaku&video_id=1'
```

**预期结果**: 
```json
{
  "code": 0,
  "data": {
    "danmakus": [...],
    "page_info": {...}
  }
}
```

**权限信息**:
- 权限等级: 1（游客）
- 权限检查: ✅ 通过
- 请求: GET 类命令
- token: 无

---

### ❌ 测试 2: 游客发送弹幕（权限=1，拒绝）

**请求**:
```bash
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -H 'Content-Type: application/json' \
  -d '{
    "cmd": {
      "service": "send_danmaku",
      "video_id": 1,
      "content": "这是一条弹幕"
    }
  }'
```

**预期结果**:
```json
{
  "code": 4003,
  "msg": "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2",
  "data": null
}
```

**权限信息**:
- 权限等级: 1（游客）
- 权限检查: ❌ 失败（需要 >= 2）
- 请求: SEND 类命令
- token: 无

---

### ✅ 测试 3: 登录用户获取评论（权限>=2，允许）

**请求**:
```bash
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -H 'Content-Type: application/json' \
  -d '{
    "auth": {
      "access_token": "your_valid_token_here"
    },
    "cmd": {
      "service": "get_comment",
      "video_id": 1,
      "page": 1,
      "qty": 10
    }
  }'
```

**预期结果**:
```json
{
  "code": 0,
  "data": {
    "comments": [...],
    "page_info": {...}
  }
}
```

**权限信息**:
- 权限等级: >= 2（登录用户）
- 权限检查: ✅ 通过
- 请求: GET 类命令
- token: 有效

---

### ✅ 测试 4: 登录用户发送评论（权限>=2，允许）

**请求**:
```bash
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -H 'Content-Type: application/json' \
  -d '{
    "auth": {
      "access_token": "your_valid_token_here"
    },
    "cmd": {
      "service": "send_comment",
      "video_id": 1,
      "content": "很不错的视频！"
    }
  }'
```

**预期结果**:
```json
{
  "code": 0,
  "data": {
    "comment_id": 12345,
    "content": "很不错的视频！",
    "created_at": "2026-09-19T02:10:00Z"
  }
}
```

**权限信息**:
- 权限等级: >= 2（登录用户）
- 权限检查: ✅ 通过
- 请求: SEND 类命令
- token: 有效

---

### ✅ 测试 5: 登录用户获取视频（权限>=2，允许）

**请求**:
```bash
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=get_video&video_id=1&access_token=your_valid_token'
```

**预期结果**:
```json
{
  "code": 0,
  "data": {
    "video_id": 1,
    "title": "视频标题",
    "description": "视频描述",
    "views": 12345,
    "likes": 678
  }
}
```

---

### ✅ 测试 6: 登录用户发布视频（权限>=2，允许）

**请求**:
```bash
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -H 'Content-Type: application/json' \
  -d '{
    "auth": {
      "access_token": "your_valid_token_here"
    },
    "cmd": {
      "service": "add_video",
      "title": "我的第一个视频",
      "description": "这是一个测试",
      "url": "https://example.com/video.mp4"
    }
  }'
```

**预期结果**:
```json
{
  "code": 0,
  "data": {
    "video_id": 12345,
    "user_id": 123,
    "title": "我的第一个视频",
    "status": "published"
  }
}
```

---

### ✅ 测试 7: 浏览主页 - 最新（权限=1，允许）

**请求**:
```bash
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=home_new&page=1&qty=10'
```

**预期结果**:
```json
{
  "code": 0,
  "data": {
    "videos": [...],
    "page_info": {...}
  }
}
```

**权限信息**:
- 权限等级: 1（游客）
- 权限检查: ✅ 通过
- 请求: GET 类命令（home_new）
- token: 无

---

### ✅ 测试 8: 浏览主页 - 热门（权限=1，允许）

**请求**:
```bash
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=home_hot&page=1&qty=10'
```

**预期结果**: 热门视频列表

---

## 📊 测试结果总结

### 预期的权限规则

| Service | 类型 | 权限要求 | 游客(1) | 用户(2+) |
|---------|------|---------|--------|---------|
| home_new | GET | >= 1 | ✅ | ✅ |
| home_hot | GET | >= 1 | ✅ | ✅ |
| home_recommend | GET | >= 1 | ✅ | ✅ |
| home_city | GET | >= 1 | ✅ | ✅ |
| home_category | GET | >= 1 | ✅ | ✅ |
| home_featured | GET | >= 1 | ✅ | ✅ |
| home_search | GET | >= 1 | ✅ | ✅ |
| view | GET | >= 1 | ✅ | ✅ |
| get_video | GET | >= 1 | ✅ | ✅ |
| add_video | ADD | >= 2 | ❌ | ✅ |
| publish_video | ADD | >= 2 | ❌ | ✅ |
| get_comment | GET | >= 1 | ✅ | ✅ |
| send_comment | SEND | >= 2 | ❌ | ✅ |
| publish_comment | SEND | >= 2 | ❌ | ✅ |
| get_danmaku | GET | >= 1 | ✅ | ✅ |
| send_danmaku | SEND | >= 2 | ❌ | ✅ |

---

## 🔍 检查点

### 游客权限测试（权限=1）
- [ ] `home_*` 命令都能访问
- [ ] `get_*` 命令都能访问
- [ ] `view` 命令能访问
- [ ] `send_*` 命令都被拒绝（返回 4003）
- [ ] `add_*` 命令都被拒绝（返回 4003）
- [ ] `publish_*` 命令都被拒绝（返回 4003）

### 登录用户权限测试（权限>=2）
- [ ] 所有 GET 命令能访问
- [ ] 所有 SEND 命令能访问
- [ ] 所有 ADD 命令能访问
- [ ] 所有命令都返回业务正常结果

### 错误信息检查
- [ ] 权限不足时返回 code: 4003
- [ ] 错误消息包含 "[🌐 GATEWAY]" 标记
- [ ] 错误消息包含 "❌️" 符号

---

## 🚀 快速测试脚本

### Bash 脚本
```bash
#!/bin/bash

# 测试 1: 游客获取弹幕
echo "Test 1: 游客获取弹幕"
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=get_danmaku&video_id=1'
echo -e "\n"

# 测试 2: 游客发送弹幕（应该失败）
echo "Test 2: 游客发送弹幕（应该失败）"
curl -X POST 'http://127.0.0.1:8080/api/v2/video/gateway' \
  -H 'Content-Type: application/json' \
  -d '{"cmd": {"service": "send_danmaku", "video_id": 1, "content": "test"}}'
echo -e "\n"

# 测试 3: 浏览主页
echo "Test 3: 浏览主页"
curl 'http://127.0.0.1:8080/api/v2/video/gateway?service=home_new'
echo -e "\n"
```

---

## 📝 故障排除

### 问题 1: 返回 404
**原因**: 网关地址错误  
**解决**: 检查 `http://127.0.0.1:8080/api/v2/video/gateway`

### 问题 2: 返回 401 + "缺少登录认证"
**原因**: 一些网关要求必须有 auth 信息  
**解决**: 尝试发送 `auth` 字段（即使为空）

### 问题 3: 所有请求都返回 4003
**原因**: 权限上下文未正确初始化  
**解决**: 检查网关层权限初始化代码

---

**祝测试顺利！** 🎉

