@echo off
REM 快速生成所有 Port Adapter 文件
REM Windows批处理脚本

setlocal enabledelayedexpansion

REM 定义所有需要生成的 adapter
REM 格式: module:action module:action ...

set "items=^
collect:check collect:del collect:get collect:list collect:manage collect:stat ^
comment:add comment:check comment:del comment:get comment:list comment:manage comment:stat ^
danmaku:add danmaku:check danmaku:del danmaku:get danmaku:list danmaku:manage danmaku:stat ^
dislike:add dislike:del dislike:list dislike:manage dislike:stat ^
hotlist:add hotlist:check hotlist:del hotlist:get hotlist:list hotlist:manage hotlist:stat ^
like:add like:check like:del like:get like:list like:manage like:stat ^
recommend:add recommend:check recommend:del recommend:get recommend:list recommend:manage recommend:stat ^
report:add report:check report:del report:get report:list report:manage report:stat ^
share:add share:check share:del share:get share:list share:manage share:stat"

setlocal enabledelayedexpansion
set count=0

for %%item in (%items%) do (
    for /f "tokens=1,2 delims=:" %%a in ("%%item") do (
        set "module=%%a"
        set "action=%%b"
        set "basepath=repo_adapter\src\video\!module!"
        set "filepath=!basepath!\!action!_port.rs"
        
        if not exist "!basepath!" mkdir "!basepath!"
        
        if not exist "!filepath!" (
            (
                echo // repo_adapter/src/video/!module!/!action!_port.rs  -- Port Adapter
                echo // 2026/8/8 Created.
                echo.
                echo ////////
                echo.
                echo use anyhow::Result;
                echo use async_trait::async_trait;
                echo use cola_data::cola_video::port::!module!::!action!::*;
                echo.
                echo ////////
                echo.
                echo #[derive(Debug, Default, Clone)]
                echo pub struct !module!_!action!_PortAdapter;
                echo.
                echo #[async_trait]
                echo impl *Port for !module!_!action!_PortAdapter {
                echo     // TODO: 实现具体的数据库操作逻辑
                echo }
                echo.
                echo //////// END
            ) > "!filepath!"
            echo Created: !filepath!
            set /a count+=1
        )
    )
)

echo.
echo 完成！生成了 %count% 个 Port Adapter 文件
pause
