# PowerShell 脚本 - 批量生成所有 Port Adapter 文件

# 定义所有需要生成的 module:action 对
$adapters = @(
    'collect:check', 'collect:del', 'collect:get', 'collect:list', 'collect:manage', 'collect:stat',
    'comment:add', 'comment:check', 'comment:del', 'comment:get', 'comment:list', 'comment:manage', 'comment:stat',
    'danmaku:add', 'danmaku:check', 'danmaku:del', 'danmaku:get', 'danmaku:list', 'danmaku:manage', 'danmaku:stat',
    'dislike:add', 'dislike:del', 'dislike:list', 'dislike:manage', 'dislike:stat',
    'hotlist:add', 'hotlist:check', 'hotlist:del', 'hotlist:get', 'hotlist:list', 'hotlist:manage', 'hotlist:stat',
    'like:add', 'like:check', 'like:del', 'like:get', 'like:list', 'like:manage', 'like:stat',
    'recommend:add', 'recommend:check', 'recommend:del', 'recommend:get', 'recommend:list', 'recommend:manage', 'recommend:stat',
    'report:add', 'report:check', 'report:del', 'report:get', 'report:list', 'report:manage', 'report:stat',
    'share:add', 'share:check', 'share:del', 'share:get', 'share:list', 'share:manage', 'share:stat'
)

$count = 0

foreach ($adapter in $adapters) {
    $parts = $adapter -split ':'
    $module = $parts[0]
    $action = $parts[1]
    
    $basePath = "repo_adapter\src\video\$module"
    $filePath = "$basePath\${action}_port.rs"
    
    # 创建目录
    if (!(Test-Path $basePath)) {
        New-Item -ItemType Directory -Path $basePath | Out-Null
    }
    
    # 检查文件是否已存在
    if (!(Test-Path $filePath)) {
        # 创建文件内容
        $content = @"
// repo_adapter/src/video/$module/${action}_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;

// TODO: 导入对应的 Port trait
// use cola_data::cola_video::port::$module::$action::*;

////////

/// # [ADAPTER] - $module $action adapter
#[derive(Debug, Default, Clone)]
pub struct ${module}${action}PortAdapter;

// TODO: 实现对应的 Port trait
// 示例结构（需要根据实际 trait 填充方法）：
// #[async_trait]
// impl SomePort for ${module}${action}PortAdapter {
//     async fn some_method(&self, ...) -> Result<...> {
//         // 实现具体的数据库操作逻辑
//         todo!()
//     }
// }

//////// END
"@
        
        $content | Out-File -FilePath $filePath -Encoding UTF8
        Write-Host "Created: $filePath"
        $count++
    }
}

Write-Host ""
Write-Host "完成！共生成 $count 个 Port Adapter 文件"
Write-Host ""
Write-Host "后续步骤："
Write-Host "1. 编辑每个文件，添加正确的 trait 导入"
Write-Host "2. 实现对应的 trait 方法"
Write-Host "3. 在对应模块的 mod.rs 中添加: pub mod ${action}_port;"
