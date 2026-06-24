taskkill /F /IM short-video.exe



# Linux / Mac SQLX 编译前检查
export DATABASE_URL=postgres://user:password@localhost:5432/mydb

# Windows SQLX 编译前检查
cargo install sqlx-cli --no-default-features --features postgres

setx DATABASE_URL "postgres://postgres:123456@localhost:5432/live_2026"

# 分析依赖包大小
cargo bloat --release --crates