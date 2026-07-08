



# 2026-07-07 18:02
version: 0.1.3
task:
- 我用VIDEO模块复制出来写了一个IM模块, 请帮我完善它
- 我已经在cola_data/im里面写好了数据表的基本结构, 但是command 创建命令还没写, 你帮我补全
- 常见的接口我已经在gate_http/im 里面的业务网关im_gateway写的差不多了,基本上就是联系人的同步 CRUD,
- 业务模型就是联系人/联系人添加请求(临时会话)/用户资料卡片(从cola_user自动转换过来, 绑定user模块的id)/离线消息的增量拉取/聊天设置/ 聊天会话 等
- 数据模型我写在了cola_data/im/entity/里面了, 你可以适当加一些字段, 但是不能删我的
- 你用我项目的代码风格, 补全 cola_data/im/command 数据创建的命令, 补全from/into 函数, 补全 构建new函数,
- 消息数据先存在pg, repo我还没写, 你帮我按照我的代码风格补充, port(trait) 我还没写, 你照着IM网关的macth路由补全, 再写adapter函数实现,repo除了pg,还要额外写一个MongoDB的实现
- 所有中文注释都要用utf8,不能用gbk.
- mango的连接池和配置在app_config模块已经有了,
- im使用乐观模式, 离线消息/联系人都使用增量拉取,
- websocket不在本系统, 所以本项目只负责cms差不多的内容