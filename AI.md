# 2026-08-10 
任务：拆散 repo_adapter 中的大型 AppContext 构造器，将各业务模块的 Port 初始化移动到对应模块内部。

背景：
当前 repo_adapter/src/lib.rs 中 build_app_context() 负责构造整个系统所有模块的 AppContext。

目前结构类似：

repo_adapter/src/lib.rs

pub fn build_app_context() -> AppContext {
    let auth = AuthServicePorts { ... };
    let user = ColaUserPort { ... };
    let video = ColaVideoPort { ... };
    let live = ColaLivePort { ... };
    let gis = ColaGisPort { ... };
    let market = ColaMarketPort { ... };
    let im = ColaImPort { ... };
    let three = ColaThreePort { ... };

    AppContext::default(...)
}

该文件已经变成大型 wiring 文件，不利于维护。


目标：
保持现有 Port / Adapter / Trait 架构不变，只拆分 Context Builder。

不要：
1. 不要合并 trait。
2. 不要降低 Port 粒度。
3. 不要改变 AppContext 结构。
4. 不要修改业务逻辑。
5. 不要改变现有 Adapter 实现。


要求：

一、每个业务模块提供自己的 builder 方法

例如：

repo_adapter/src/video/mod.rs

增加：

pub fn build_video_port() -> ColaVideoPort {
    ColaVideoPort {
        ...
    }
}


类似：

auth:

pub fn build_auth_port() -> AuthServicePorts


user:

pub fn build_user_port() -> ColaUserPort


gis:

pub fn build_gis_port() -> ColaGisPort


live:

pub fn build_live_port() -> ColaLivePort


market:

pub fn build_market_port() -> ColaMarketPort


music:

pub fn build_music_port() -> ColaMusicPort


three:

pub fn build_three_port() -> ColaThreePort


im:

pub fn build_im_port() -> ColaImPort


二、调整 repo_adapter/src/lib.rs

最终只保留总装配：

示例：

pub fn build_app_context() -> AppContext {

    AppContext::default(
        auth::build_auth_port(),
        gis::build_gis_port(),
        live::build_live_port(),
        market::build_market_port(),
        music::build_music_port(),
        three::build_three_port(),
        user::build_user_port(),
        video::build_video_port(),
        im::build_im_port(),
    )

}


三、目录要求

调整后：

repo_adapter

├── lib.rs

├── auth
│   └── mod.rs
│
├── user
│   └── mod.rs
│
├── video
│   └── mod.rs
│
├── live
│   └── mod.rs
│
├── gis
│   └── mod.rs
│
├── market
│   └── mod.rs
│
├── music
│   └── mod.rs
│
├── three
│   └── mod.rs
│
└── im
    └── mod.rs


四、迁移规则

原 build_app_context 中：

例如：

let video = ColaVideoPort {
    add: Arc::new(...),
    like: ...,
    comment: ...,
};

完整移动到：

video::build_video_port()


不要复制代码。

移动完成后删除 lib.rs 中对应初始化代码。


五、保持编译通过

执行：

cargo check

修复：

- use 路径
- module visibility
- 循环引用
- trait import

直到 workspace 编译通过。


六、代码风格

保持当前项目风格：

- Rust 2024
- async_trait 保留
- Arc<dyn Trait> 保留
- 不引入新的依赖
- 不增加宏
- 每个文件控制复杂度


最终结果：

repo_adapter/src/lib.rs 从大型配置文件变成简单入口。

业务模块自己负责自己的 Adapter 装配。

AppContext 仍然作为唯一依赖入口。



