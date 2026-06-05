# 1. 初始化 Git 仓库（如果尚未初始化）
git init

# 2. 创建并配置 .gitignore（参考之前的 Rust 配置）
# （手动创建或用命令生成，例如：curl -o .gitignore https://www.toptal.com/developers/gitignore/api/rust,visualstudiocode）

# 3. 检查文件状态（确认 .gitignore 生效）
git status

# 4. 暂存文件（⚠️ 建议用具体路径而非 git add .，避免误加编译产物）
# 使用以下命令逐个目录添加：
git add src/ app/ api/ cola_*/ gateway/ health/ im/ kits/ network/ repo/ migrations/ assets/
# 注意：target/、build_output*.txt 等已被 .gitignore 忽略

# 修改.gitignore删掉旧屏蔽规则
# 清除仓库缓存重新扫描：
# bash
# 运行
git rm -r --cached .
git add .

# 5. 提交到本地仓库
git commit -m "Initial commit: Add Rust project source code"

# 6. （可选）关联远程仓库（首次推送时需要）
git remote -v
git remote add origin https://github.com/CHAT-00001/Cola-IM-SERVICE-CMS.git

# 7. （可选）推送到远程仓库
git push -u origin main

# 8. 覆盖推送（⚠️ 仅在需要强制覆盖远程时使用）
git push -u origin main --force
