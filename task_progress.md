# cola_three 第三方服务配置模块 - 实现清单

## Phase 1: cola_data 数据结构层
- [x] cola_data/src/three/ 目录结构 + mod.rs
- [x] entity: three_type / three_vendor / three_config / three_biz_binding
- [x] command: type/vendor/config/binding
- [x] info: type/vendor/config/binding
- [x] port: type/vendor/config/binding + ColaThreePort

## Phase 2: repo 数据仓储层
- [x] repo/src/three/ 目录 + PG 实现

## Phase 3: repo_adapter 适配器层
- [x] repo_adapter/src/three/ 端口注入

## Phase 4: cola_three 业务模块
- [x] cola_three/Cargo.toml + lib.rs
- [x] model/command + model/vo
- [x] case: type/vendor/config/binding
- [x] api: type/vendor/config/binding

## Phase 5: 基础设施更新
- [x] AppContext 加入 ColaThreePort
- [x] build_app_context() 注入 three 适配器
- [x] Cargo.toml workspace 注册 cola_three
- [x] gate_http 路由注册（admin/three gateway）

## Phase 6: 数据库迁移
- [x] three_type 预设数据 SQL
- [x] three_vendor 预设数据 SQL
- [x] three_config 表 + three_biz_binding 表
