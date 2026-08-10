# 更新所有模块的 mod.rs 文件

$modules = @(
    'danmaku',
    'dislike', 
    'hotlist',
    'like',
    'recommend',
    'report',
    'share'
)

foreach ($module in $modules) {
    $basePath = "repo_adapter/src/video/$module"
    $modFilePath = "$basePath/mod.rs"
    
    # 生成 mod 声明
    if ($module -eq 'dislike') {
        $modLines = @(
            'pub mod add_port;'
            'pub mod del_port;'
            'pub mod list_port;'
            'pub mod manage_port;'
            'pub mod stat_port;'
        )
    } else {
        $modLines = @(
            'pub mod add_port;'
            'pub mod check_port;'
            'pub mod del_port;'
            'pub mod get_port;'
            'pub mod list_port;'
            'pub mod manage_port;'
            'pub mod stat_port;'
        )
    }
    
    $modDeclare = $modLines -join "`n"
    
    $content = @"
// repo_adapter/src/video/$module/mod.rs
// Port Adapter - 视频 - $module - 模块
// 2026/8/8 Created.

////////

$modDeclare

//////// END
"@
    
    if (Test-Path $modFilePath) {
        $existing = Get-Content $modFilePath -Raw
        if ($existing -match "pub mod add_port;") {
            Write-Host "Already updated: $modFilePath"
        } else {
            Set-Content -Path $modFilePath -Value $content -Encoding UTF8
            Write-Host "Updated: $modFilePath"
        }
    } else {
        Set-Content -Path $modFilePath -Value $content -Encoding UTF8
        Write-Host "Created: $modFilePath"
    }
}

Write-Host "Done!"
