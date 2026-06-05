## db.sh -- 数据库命令


## 1. 安装 sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres


## 2. 创建迁移的sql文件

# user_count
sqlx migrate add user_count_tables
# user_auth
sqlx migrate add user_auth_tables

## AUTH - 验证中心
# 验证 - 设备
sqlx migrate add auth_device_table
# 验证 - token
sqlx migrate add auth_session_table


## VIDEO - 短视频
#  创建 video表
sqlx migrate add video_table


## USER - 用户
# 用户表
sqlx migrate add user_table
# 用户统计表
sqlx migrate add user_count_table
# 用户认证表
sqlx migrate add user_auth_table


## 3. 执行迁移
sqlx migrate run