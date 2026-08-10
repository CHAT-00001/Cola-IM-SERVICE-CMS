#!/bin/bash

# 创建所有 Port 实现文件

modules=(
    "collect:check:del:get:list:manage:stat"
    "comment:check:del:get:list:manage:stat"
    "danmaku:add:check:del:get:list:manage:stat"
    "dislike:add:del:list:manage:stat"
    "hotlist:add:check:del:get:list:manage:stat"
    "like:add:check:del:get:list:manage:stat"
    "recommend:add:check:del:get:list:manage:stat"
    "report:add:check:del:get:list:manage:stat"
    "share:add:check:del:get:list:manage:stat"
)

for module_spec in "${modules[@]}"; do
    IFS=':' read -ra parts <<< "$module_spec"
    module="${parts[0]}"
    
    for action in "${parts[@]:1}"; do
        filepath="repo_adapter/src/video/${module}/${action}_port.rs"
        
        if [ ! -f "$filepath" ]; then
            mkdir -p "repo_adapter/src/video/$module"
            
            cat > "$filepath" << EOF
// repo_adapter/src/video/$module/${action}_port.rs
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;

////////

#[derive(Debug, Default, Clone)]
pub struct ${module}${action}PortAdapter;

// TODO: 实现对应的 Port trait
// 在此添加 #[async_trait] impl 块

//////// END
EOF
            echo "✅ Created: $filepath"
        fi
    done
done

echo "Done!"
