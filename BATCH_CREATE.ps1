# PowerShell 脚本 - 批量创建所有 Port Adapter 文件

$modules = @{
    'comment' = @{
        'del' = 'VideoCommentDelPort'
        'get' = 'VideoCommentGetPort'
        'list' = 'VideoCommentListPort'
        'manage' = 'VideoCommentManagePort'
        'stat' = 'VideoCommentStatPort'
    }
    'danmaku' = @{
        'add' = 'DanmakuAddPort'
        'check' = 'DanmakuCheckPort'
        'del' = 'DanmakuDelPort'
        'get' = 'DanmakuGetPort'
        'list' = 'DanmakuListPort'
        'manage' = 'DanmakuManagePort'
        'stat' = 'DanmakuStatPort'
    }
    'dislike' = @{
        'add' = 'DislikeAddPort'
        'del' = 'DislikeDelPort'
        'list' = 'DislikeListPort'
        'manage' = 'DislikeManagePort'
        'stat' = 'VideoDislikeStatPort'
    }
    'hotlist' = @{
        'add' = 'HotlistAddPort'
        'check' = 'VideoHotlistCheckPort'
        'del' = 'VideoHotlistDelPort'
        'get' = 'VideoHotlistGetPort'
        'list' = 'VideoHotlistListPort'
        'manage' = 'VideoHotlistManagePort'
        'stat' = 'VideoHotlistStatPort'
    }
    'like' = @{
        'add' = 'LikeAddPort'
        'check' = 'LikeCheckPort'
        'del' = 'LikeDelPort'
        'get' = 'LikeGetPort'
        'list' = 'LikeListPort'
        'manage' = 'LikeManagePort'
        'stat' = 'LikeStatPort'
    }
    'recommend' = @{
        'add' = 'AddPort'
        'check' = 'VideoRecommendCheckPort'
        'del' = 'VideoRecommendDelPort'
        'get' = 'VideoRecommendGetPort'
        'list' = 'VideoRecommendListPort'
        'manage' = 'VideoRecommendManagePort'
        'stat' = 'VdieoRecommendStatPort'
    }
    'report' = @{
        'add' = 'VideoReportAddPort'
        'check' = 'VideoReportCheckPort'
        'del' = 'VideoReportDelPort'
        'get' = 'VideoReportGetPort'
        'list' = 'VideoReportListPort'
        'manage' = 'ReportManagePort'
        'stat' = 'ReportStatPort'
    }
    'share' = @{
        'add' = 'VideoShareAddPort'
        'check' = 'VideoShareCheckPort'
        'del' = 'VideoShareDelPort'
        'get' = 'VideoShareGetPort'
        'list' = 'VideoShareListPort'
        'manage' = 'VideoShareManagePort'
        'stat' = 'VideoShareStatPort'
    }
}

$count = 0

foreach ($module in $modules.Keys) {
    foreach ($action in $modules[$module].Keys) {
        $trait = $modules[$module][$action]
        
        $basePath = "repo_adapter\src\video\$module"
        $filePath = "$basePath\${action}_port.rs"
        
        if (!(Test-Path $basePath)) {
            New-Item -ItemType Directory -Path $basePath -Force | Out-Null
        }
        
        if (!(Test-Path $filePath)) {
            $content = @"
// repo_adapter/src/video/$module/${action}_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::$module::$action::$trait;

////////

/// # [ADAPTER] - $module $action
#[derive(Debug, Default, Clone)]
pub struct ${module}${action}PortAdapter;

#[async_trait]
impl $trait for ${module}${action}PortAdapter {
    // TODO: 实现具体的业务逻辑
}

//////// END
"@
            
            $content | Out-File -FilePath $filePath -Encoding UTF8
            Write-Host "Created: $filePath"
            $count++
        }
    }
}

Write-Host "`nTotal created: $count files"
