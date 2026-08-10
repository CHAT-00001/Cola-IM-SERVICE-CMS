# 快速修复所有 Struct 名称

$map = @{
    'like' = @{
        'add' = 'LikeAddPortAdapter'
        'check' = 'LikeCheckPortAdapter'
        'del' = 'LikeDelPortAdapter'
        'get' = 'LikeGetPortAdapter'
        'list' = 'LikeListPortAdapter'
        'manage' = 'LikeManagePortAdapter'
        'stat' = 'LikeStatPortAdapter'
    }
    'comment' = @{
        'del' = 'CommentDelPortAdapter'
        'get' = 'CommentGetPortAdapter'
        'list' = 'CommentListPortAdapter'
        'manage' = 'CommentManagePortAdapter'
        'stat' = 'CommentStatPortAdapter'
    }
}

foreach ($module in $map.Keys) {
    foreach ($action in $map[$module].Keys) {
        $filePath = "repo_adapter/src/video/$module/${action}_port.rs"
        $expectedStruct = $map[$module][$action]
        
        if (Test-Path $filePath) {
            $content = Get-Content $filePath -Raw -Encoding UTF8
            
            # 提取 trait 名称
            if ($content -match 'impl (\w+) for') {
                $traitName = $matches[1]
                
                # 替换 struct 定义和 impl 块
                $newContent = $content -replace 'pub struct \w+PortAdapter;', "pub struct $expectedStruct;"
                $newContent = $newContent -replace "impl $traitName for \w+PortAdapter", "impl $traitName for $expectedStruct"
                
                # 如果有变化则保存
                if ($newContent -ne $content) {
                    Set-Content -Path $filePath -Value $newContent -Encoding UTF8
                    Write-Host "Fixed: $filePath -> $expectedStruct"
                }
            }
        }
    }
}

Write-Host "Done!"
