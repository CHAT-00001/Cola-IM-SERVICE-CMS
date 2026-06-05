# DEVELOPMENT DOC


# 文件目录架构

## API 路由器/HANDLER

## APP 应用状态机
- 数据库连接池/应用状态/多服务启动...

## CORE/XXXX 应用核心/应用名称  --LIB
1. APP 应用层 USE CASE 应用权限编排 / 组装到AppDate统一响应壳
    - ADD 创建/发布 相关的应用
    - GET 通用的查看接口
    - CHANGE 修改/编辑
    - DEL 删除
    - HOME 抽象的前台应用
    - AT 关于我的应用
    - TA 关于TA的应用
    - 
2. Biz 逻辑层 LOGIC 逻辑流程编排 / 先xx再xx然后xx / 调用外部工具函数构建数据 / 使用服务端口Port 查找保存数据干净的repository数据，返回数据给 APP 层。
3. Aessmebler 组装层  拼接组装多维数据，BIZ直接调用
4. Port 服务端口层 使用rust的trait提供函数给后级lib bridge 库使用
5. kits 其他工具（备用）

## BRIDGE/xxxx 桥接/应用名称  -- LIB
1. adapter 适配器 自适应 repository 的数据 CACHE > DB > GRPC> MORK..仓储层脱离程序不会崩溃     


## DATA/xxxx 数据/应用名称  --LIB
1. command - 创建命令
2. entity - 数据表实体  仅数据表映射
3. info - 数据信息 从实体构造数据元对象
4. model - 数据视图对象 从info拿到元数据信息组装成视图对象XxxxVO 对象
5. 

## REPO/xxxx 仓储/应用名称 -- LIB
1. redis - 缓存
2. pg - postgreSQL仓储层
3. grpc - 远程调用（未开发）
4. mock - 模拟数据（未开发）

## 响应顺序：
Entity
↓
Info（Redis缓存）
↓
VideoVo（聚合展示）
↓
Response
↓
AppData