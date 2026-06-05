# Project Structure

> **更新时间：** 2026-05-22 15:10
> **架构核心：** 采用业务与功能“双垂直”架构，方便快速插拔、扩展与收缩业务模块。

```text
├── src/                  # BOOT 启动机
│   ├── main/             # MAIN 入口
│   └── app/              # APP 启动配置与生命周期
├── app/                  # APP 全局基础
│   ├── config/           # 全局配置
│   ├── state/            # 全局状态管理
│   ├── network/          # 外部网络基础驱动
│   ├── common/           # 公共模块（备用）
│   └── lib/              # 全局通用库
├── api/xxxx/             # api/app_name - transport layer（传输层/API接口）
│   ├── handler/          # API 处理器
│   │   └── router/       # App 应用内部路由
│   ├── router/           # Api 根路由
│   └── lib/              # 传输层专用库
├── core/xxxx/            # core/app_name - core layer（核心层）
│   ├── app/              # Use Case（用例），负责注入 AppDate 统一响应壳
│   ├── biz/              # 业务流水线（核心业务逻辑）
│   ├── assembler/        # 数据组装器（DTO/VO 转换）
│   ├── port/             # 服务端口（与 bridge 层对接的抽象定义）
│   └── lib/              # 核心层专用库
├── bridge/xxxx/          # bridge/app_name - bridge layer（桥接层）
│   ├── adapter/          # 数据插头适配器
│   └── lib/              # 桥接层专用库
├── data/xxxx/            # data/app_name - data layer（数据层）
│   ├── agg/              # 聚合视图模型
│   ├── command/          # 命令模型（如发布内容命令等 CQS 实践）
│   ├── entity/           # 数据库表实体对象
│   ├── event/            # 消息事件驱动模型
│   ├── model/            # VO 视图/传输模型
│   └── lib/              # 数据层专用库
├── gateway/              # gateway layer（网关层）（可选）
│   └── lib/              # 网关层专用库
├── im/xxxx/              # im client（IM 客户端业务模块）
│   └── lib/              # IM 专用库
├── health/               # 应用健康报告（后级微服务/API 生效）
│   └── lib/              # 健康检查专用库
├── repo/xxxx/         # storage/app_name - repository layer（仓储层）
│   ├── --/               # （备用）
│   ├── redis/            # Redis 缓存实现
│   ├── pg/               # PostgreSQL 数据库实现
│   ├── grpc/             # 外部 RPC 程序调用
│   ├── mock/             # 单元测试模拟数据
│   └── lib/              # 仓储层专用库
├── README.md
└── ARCHITECTURE.md