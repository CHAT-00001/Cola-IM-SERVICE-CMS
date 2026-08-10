# 修复所有 Port Adapter 文件中的 Struct 名称

$basePath = "repo_adapter/src/video"

# 获取所有 _port.rs 文件
Get-ChildItem -Path $basePath -Recurse -Filter "*_port.rs" | ForEach-Object {
    $filePath = $_.FullName
    $content = Get-Content $filePath -Raw -Encoding UTF8
    
    # 提取模块名和操作名
    if ($filePath -match "\\video\\([^\\]+)\\([^\\]+)_port\.rs$") {
        $module = $matches[1]
        $action = $matches[2]
        
        # 构建期望的 Struct 名称 (PascalCase)
        $modulePascal = (Get-Culture).TextInfo.ToTitleCase($module)
        $actionPascal = (Get-Culture).TextInfo.ToTitleCase($action)
        
        $expectedName = "${modulePascal}${actionPascal}PortAdapter"
        
        # 检查是否需要修复
        if ($content -match "pub struct ([^ ]+)PortAdapter") {
            $currentName = $matches[1] + "PortAdapter"
            
            if ($currentName -ne $expectedName) {
                # 修复 Struct 定义
                $newContent = $content -replace "pub struct \w+PortAdapter", "pub struct $expectedName"
                
                # 修复 impl 块
                $newContent = $newContent -replace "impl \w+ for \w+PortAdapter", "impl $($module | Select-Object -ExpandProperty Chars 0)$($modulePascal.Substring(1)) for $expectedName"
                
                # 实际上让我们更精确地做
                # 需要从 trait 名称中提取
                if ($content -match "use cola_data::cola_video::port::([^:]+)::([^:]+)::([^;]+);") {
                    $traitName = $matches[3]
                    $newContent = $content -replace "pub struct \w+PortAdapter", "pub struct $expectedName"
                    $newContent = $newContent -replace "impl $traitName for \w+PortAdapter", "impl $traitName for $expectedName"
                    
                    Set-Content -Path $filePath -Value $newContent -Encoding UTF8
                    Write-Host "Fixed: $filePath -> $expectedName"
                }
            }
        }
    }
}

Write-Host "Struct name fixing complete!"
