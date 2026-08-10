#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
快速生成所有 Port Adapter 文件
"""

import os
from pathlib import Path

# 所有需要生成的 adapter
adapters = [
    # collect
    ('collect', 'check'),
    ('collect', 'del'),
    ('collect', 'get'),
    ('collect', 'list'),
    ('collect', 'manage'),
    ('collect', 'stat'),
    
    # comment
    ('comment', 'add'),
    ('comment', 'check'),
    ('comment', 'del'),
    ('comment', 'get'),
    ('comment', 'list'),
    ('comment', 'manage'),
    ('comment', 'stat'),
    
    # danmaku
    ('danmaku', 'add'),
    ('danmaku', 'check'),
    ('danmaku', 'del'),
    ('danmaku', 'get'),
    ('danmaku', 'list'),
    ('danmaku', 'manage'),
    ('danmaku', 'stat'),
    
    # dislike
    ('dislike', 'add'),
    ('dislike', 'del'),
    ('dislike', 'list'),
    ('dislike', 'manage'),
    ('dislike', 'stat'),
    
    # hotlist
    ('hotlist', 'add'),
    ('hotlist', 'check'),
    ('hotlist', 'del'),
    ('hotlist', 'get'),
    ('hotlist', 'list'),
    ('hotlist', 'manage'),
    ('hotlist', 'stat'),
    
    # like
    ('like', 'add'),
    ('like', 'check'),
    ('like', 'del'),
    ('like', 'get'),
    ('like', 'list'),
    ('like', 'manage'),
    ('like', 'stat'),
    
    # recommend
    ('recommend', 'add'),
    ('recommend', 'check'),
    ('recommend', 'del'),
    ('recommend', 'get'),
    ('recommend', 'list'),
    ('recommend', 'manage'),
    ('recommend', 'stat'),
    
    # report
    ('report', 'add'),
    ('report', 'check'),
    ('report', 'del'),
    ('report', 'get'),
    ('report', 'list'),
    ('report', 'manage'),
    ('report', 'stat'),
    
    # share
    ('share', 'add'),
    ('share', 'check'),
    ('share', 'del'),
    ('share', 'get'),
    ('share', 'list'),
    ('share', 'manage'),
    ('share', 'stat'),
]

count = 0

for module, action in adapters:
    base_path = f'repo_adapter/src/video/{module}'
    file_path = f'{base_path}/{action}_port.rs'
    
    # 创建目录
    Path(base_path).mkdir(parents=True, exist_ok=True)
    
    # 检查文件是否已存在
    if not os.path.exists(file_path):
        # 生成文件内容
        content = f"""// repo_adapter/src/video/{module}/{action}_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;

// TODO: 导入对应的 Port trait
// use cola_data::cola_video::port::{module}::{action}::*;

////////

/// # [ADAPTER] - {module} {action} adapter
#[derive(Debug, Default, Clone)]
pub struct {module.capitalize()}{action.capitalize()}PortAdapter;

// TODO: 实现对应的 Port trait
// 示例结构（需要根据实际 trait 填充方法）：
// #[async_trait]
// impl SomePort for {module.capitalize()}{action.capitalize()}PortAdapter {{
//     async fn some_method(&self, ...) -> Result<...> {{
//         // 实现具体的数据库操作逻辑
//         todo!()
//     }}
// }}

//////// END
"""
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f'OK: {file_path}')
        count += 1
    else:
        print(f'SKIP: {file_path} (already exists)')

print(f'\nDone! Generated {count} adapter files')
