#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
from pathlib import Path

# 定义所有模块和对应的 trait
modules = {
    'buy': {
        'add': 'BuyAddPort',
        'check': 'BuyCheckPort',
        'del': 'BuyDelPort',
        'get': 'BuyGetPort',
        'list': 'BuyListPort',
        'manage': 'BuyManagePort',
        'stat': 'BuyStatPort',
    },
    'collect': {
        'add': 'CollectAddPort',
        'check': 'CollectCheckPort',
        'del': 'CollectDelPort',
        'get': 'CollectGetPort',
        'list': 'CollectListPort',
        'manage': 'CollectManagePort',
        'stat': 'CollectStatPort',
    },
    'comment': {
        'add': 'AddPort',
        'check': 'VideoCommentCheckPort',
        'del': 'VideoCommentDelPort',
        'get': 'VideoCommentGetPort',
        'list': 'VideoCommentListPort',
        'manage': 'VideoCommentManagePort',
        'stat': 'VideoCommentStatPort',
    },
    'danmaku': {
        'add': 'DanmakuAddPort',
        'check': 'DanmakuCheckPort',
        'del': 'DanmakuDelPort',
        'get': 'DanmakuGetPort',
        'list': 'DanmakuListPort',
        'manage': 'DanmakuManagePort',
        'stat': 'DanmakuStatPort',
    },
    'dislike': {
        'add': 'DislikeAddPort',
        'del': 'DislikeDelPort',
        'list': 'DislikeListPort',
        'manage': 'DislikeManagePort',
        'stat': 'VideoDislikeStatPort',
    },
    'hotlist': {
        'add': 'HotlistAddPort',
        'check': 'VideoHotlistCheckPort',
        'del': 'VideoHotlistDelPort',
        'get': 'VideoHotlistGetPort',
        'list': 'VideoHotlistListPort',
        'manage': 'VideoHotlistManagePort',
        'stat': 'VideoHotlistStatPort',
    },
    'like': {
        'add': 'LikeAddPort',
        'check': 'LikeCheckPort',
        'del': 'LikeDelPort',
        'get': 'LikeGetPort',
        'list': 'LikeListPort',
        'manage': 'LikeManagePort',
        'stat': 'LikeStatPort',
    },
    'recommend': {
        'add': 'AddPort',
        'check': 'VideoRecommendCheckPort',
        'del': 'VideoRecommendDelPort',
        'get': 'VideoRecommendGetPort',
        'list': 'VideoRecommendListPort',
        'manage': 'VideoRecommendManagePort',
        'stat': 'VdieoRecommendStatPort',
    },
    'report': {
        'add': 'VideoReportAddPort',
        'check': 'VideoReportCheckPort',
        'del': 'VideoReportDelPort',
        'get': 'VideoReportGetPort',
        'list': 'VideoReportListPort',
        'manage': 'ReportManagePort',
        'stat': 'ReportStatPort',
    },
    'share': {
        'add': 'VideoShareAddPort',
        'check': 'VideoShareCheckPort',
        'del': 'VideoShareDelPort',
        'get': 'VideoShareGetPort',
        'list': 'VideoShareListPort',
        'manage': 'VideoShareManagePort',
        'stat': 'VideoShareStatPort',
    },
}

def pascal_case(s):
    """Convert snake_case to PascalCase"""
    return ''.join(word.capitalize() for word in s.split('_'))

def generate_adapter_file(module, action, trait_name):
    """Generate the content of a single adapter file"""
    
    # Generate struct name
    struct_name = f"{pascal_case(module)}{pascal_case(action)}PortAdapter"
    
    content = f"""// repo_adapter/src/video/{module}/{action}_port.rs  -- 🔌 Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::{module}::{action}::{trait_name};

////////

/// # [ADAPTER] - {module} {action}
/// * `desc`: Adapter implementation for {trait_name}
#[derive(Debug, Default, Clone)]
pub struct {struct_name};

#[async_trait]
impl {trait_name} for {struct_name} {{
    // TODO: 实现具体的业务逻辑
}}

//////// END
"""
    return content

# 生成所有文件
count = 0
for module, actions in modules.items():
    for action, trait_name in actions.items():
        base_path = Path(f"repo_adapter/src/video/{module}")
        file_path = base_path / f"{action}_port.rs"
        
        content = generate_adapter_file(module, action, trait_name)
        
        # 写入文件 (覆盖)
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"✅ Generated: {file_path}")
        count += 1

print(f"\n✨ Total: {count} files regenerated!")
