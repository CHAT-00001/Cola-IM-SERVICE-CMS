# 生成所有 Port 实现文件的 PowerShell 脚本

$modules = @{
    'collect' = @('check', 'del', 'get', 'list', 'manage', 'stat')
    'comment' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'danmaku' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'dislike' = @('add', 'del', 'list', 'manage', 'stat')
    'hotlist' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'like' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'recommend' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'report' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
    'share' = @('add', 'check', 'del', 'get', 'list', 'manage', 'stat')
}

$traitMap = @{
    'comment_add' = 'AddPort'
    'comment_check' = 'VideoCommentCheckPort'
    'comment_del' = 'VideoCommentDelPort'
    'comment_get' = 'VideoCommentGetPort'
    'comment_list' = 'VideoCommentListPort'
    'comment_manage' = 'VideoCommentManagePort'
    'comment_stat' = 'VideoCommentStatPort'
    'dislike_stat' = 'VideoDislikeStatPort'
    'hotlist_check' = 'VideoHotlistCheckPort'
    'hotlist_del' = 'VideoHotlistDelPort'
    'hotlist_get' = 'VideoHotlistGetPort'
    'hotlist_list' = 'VideoHotlistListPort'
    'hotlist_manage' = 'VideoHotlistManagePort'
    'hotlist_stat' = 'VideoHotlistStatPort'
    'recommend_check' = 'VideoRecommendCheckPort'
    'recommend_del' = 'VideoRecommendDelPort'
    'recommend_get' = 'VideoRecommendGetPort'
    'recommend_list' = 'VideoRecommendListPort'
    'recommend_manage' = 'VideoRecommendManagePort'
    'recommend_stat' = 'VdieoRecommendStatPort'
    'report_add' = 'VideoReportAddPort'
    'report_check' = 'VideoReportCheckPort'
    'report_del' = 'VideoReportDelPort'
    'report_get' = 'VideoReportGetPort'
    'report_list' = 'VideoReportListPort'
    'report_manage' = 'ReportManagePort'
    'report_stat' = 'ReportStatPort'
    'share_add' = 'VideoShareAddPort'
    'share_check' = 'VideoShareCheckPort'
    'share_del' = 'VideoShareDelPort'
    'share_get' = 'VideoShareGetPort'
    'share_list' = 'VideoShareListPort'
    'share_manage' = 'VideoShareManagePort'
    'share_stat' = 'VideoShareStatPort'
}

function Get-TraitName {
    param([string]$module, [string]$action)
    
    $key = "${module}_${action}"
    if ($traitMap.ContainsKey($key)) {
        return $traitMap[$key]
    }
    
    # 默认规则
    $modCap = (Get-Culture).TextInfo.ToTitleCase($module)
    $actCap = (Get-Culture).TextInfo.ToTitleCase($action)
    return "${modCap}${actCap}Port"
}

$count = 0

foreach ($module in $modules.Keys) {
    foreach ($action in $modules[$module]) {
        $basePath = "repo_adapter/src/video/$module"
        $filePath = "$basePath/${action}_port.rs"
        
        # 创建目录
        if (!(Test-Path $basePath)) {
            New-Item -ItemType Directory -Path $basePath | Out-Null
        }
        
        # 检查文件是否存在
        if (Test-Path $filePath) {
            Write-Host "⏭️  跳过（已存在）: $filePath"
            continue
        }
        
        $traitName = Get-TraitName $module $action
        $structName = "${module}${action}PortAdapter" -replace '(\w)(\w*)', { $_.Groups[1].Value.ToUpper() + $_.Groups[2].Value.ToLower() }
        
        # 中文描述
        $moduleCn = @{
            'buy' = '购买'
            'collect' = '收藏'
            'comment' = '评论'
            'danmaku' = '弹幕'
            'dislike' = '不喜欢'
            'hotlist' = '热门'
            'like' = '点赞'
            'recommend' = '推荐'
            'report' = '举报'
            'share' = '分享'
        }[$module]
        
        $actionCn = @{
            'add' = '添加'
            'check' = '检查'
            'del' = '删除'
            'get' = '获取'
            'list' = '列表'
            'manage' = '管理'
            'stat' = '统计'
        }[$action]
        
        $content = @"
// repo_adapter/src/video/$module/${action}_port.rs  -- 🔌 视频$moduleCn - $actionCn Port 实现
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::$module::$action::$traitName;

////////

/// # [ADAPTER] - 视频$moduleCn$actionCn
/// * \`desc\`: 实现视频$moduleCn的$actionCn操作
#[derive(Debug, Default, Clone)]
pub struct ${structName};

#[async_trait]
impl $traitName for ${structName} {
    // TODO: 实现具体的数据库操作逻辑
    // 该trait方法需要根据业务需求补全实现
}

//////// END
"@
        
        $content | Out-File -FilePath $filePath -Encoding UTF8
        Write-Host "✅ 生成: $filePath"
        $count++
    }
}

Write-Host "`n📊 总共生成了 $count 个 Port 文件"
