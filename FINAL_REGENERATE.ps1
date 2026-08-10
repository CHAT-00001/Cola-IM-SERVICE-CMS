# 最终重新生成所有 Port Adapter 文件

$adapters = @(
    @{ module='like'; action='add'; trait='LikeAddPort'; struct='LikeAddPortAdapter' }
    @{ module='like'; action='check'; trait='LikeCheckPort'; struct='LikeCheckPortAdapter' }
    @{ module='like'; action='del'; trait='LikeDelPort'; struct='LikeDelPortAdapter' }
    @{ module='like'; action='get'; trait='LikeGetPort'; struct='LikeGetPortAdapter' }
    @{ module='like'; action='list'; trait='LikeListPort'; struct='LikeListPortAdapter' }
    @{ module='like'; action='manage'; trait='LikeManagePort'; struct='LikeManagePortAdapter' }
    @{ module='like'; action='stat'; trait='LikeStatPort'; struct='LikeStatPortAdapter' }
)

foreach ($adapter in $adapters) {
    $basePath = "repo_adapter/src/video/$($adapter.module)"
    $filePath = "$basePath/$($adapter.action)_port.rs"
    
    $content = @"
// repo_adapter/src/video/$($adapter.module)/$($adapter.action)_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::$($adapter.module)::$($adapter.action)::$($adapter.trait);

////////

/// # [ADAPTER] - $($adapter.module) $($adapter.action)
/// * `desc`: Adapter implementation
#[derive(Debug, Default, Clone)]
pub struct $($adapter.struct);

#[async_trait]
impl $($adapter.trait) for $($adapter.struct) {
    // TODO: 实现具体的业务逻辑
}

//////// END
"@
    
    Set-Content -Path $filePath -Value $content -Encoding UTF8
    Write-Host "✅ Created: $filePath"
}

Write-Host "Done!"
