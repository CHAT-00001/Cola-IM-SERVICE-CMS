#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# 快速生成所有 Port 实现文件

import os
from pathlib import Path

# 定义所有模块和它们对应的 Port trait actions
modules = {
    'collect': ['check', 'del', 'get', 'list', 'manage', 'stat'],
    'comment': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'danmaku': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'dislike': ['add', 'del', 'list', 'manage', 'stat'],
    'hotlist': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'like': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'recommend': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'report': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'share': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
}

# 映射module名到trait名的规则
def get_trait_name(module, action):
    """生成trait名称"""
    # buy_add -> BuyAddPort
    # dislike_add -> DislikeAddPort
    # comment_add -> AddPort (特殊情况)
    
    if module == 'comment' and action == 'add':
        return 'AddPort'
    
    # 首字母大写
    mod_cap = ''.join(word.capitalize() for word in module.split('_'))
    act_cap = action.capitalize()
    
    # 某些module需要特殊处理
    if module == 'comment' and action in ['check', 'del', 'get', 'list', 'manage', 'stat']:
        mod_cap = 'VideoComment'
    elif module == 'dislike' and action == 'stat':
        return 'VideoDislikeStatPort'
    elif module == 'hotlist' and action == 'check':
        return 'VideoHotlistCheckPort'
    elif module == 'hotlist' and action == 'del':
        return 'VideoHotlistDelPort'
    elif module == 'hotlist' and action == 'get':
        return 'VideoHotlistGetPort'
    elif module == 'hotlist' and action == 'list':
        return 'VideoHotlistListPort'
    elif module == 'hotlist' and action == 'manage':
        return 'VideoHotlistManagePort'
    elif module == 'hotlist' and action == 'stat':
        return 'VideoHotlistStatPort'
    elif module == 'recommend' and action in ['check', 'del', 'get', 'list', 'manage', 'stat']:
        mod_cap = 'VideoRecommend'
        if action == 'stat':
            return 'VdieoRecommendStatPort'  # 注意：原代码中有拼写错误
    elif module == 'report' and action == 'stat':
        return 'ReportStatPort'
    elif module == 'report':
        mod_cap = 'VideoReport'
    elif module == 'share':
        mod_cap = 'VideoShare'
    
    return f'{mod_cap}{act_cap}Port'

def generate_port_file(module, action):
    """生成单个Port文件"""
    base_path = f'repo_adapter/src/video/{module}'
    os.makedirs(base_path, exist_ok=True)
    
    filepath = f'{base_path}/{action}_port.rs'
    
    # 检查是否已存在
    if os.path.exists(filepath):
        print(f'⏭️  跳过（已存在）: {filepath}')
        return
    
    # 获取trait名
    trait_name = get_trait_name(module, action)
    
    # 获取trait路径
    trait_path = f'cola_data::cola_video::port::{module}::{action}::{trait_name}'
    
    # 创建struct名
    struct_name = f'{module.capitalize()}{action.capitalize()}PortAdapter'
    
    # 创建中文描述
    module_cn = {
        'buy': '购买',
        'collect': '收藏',
        'comment': '评论',
        'danmaku': '弹幕',
        'dislike': '不喜欢',
        'hotlist': '热门',
        'like': '点赞',
        'recommend': '推荐',
        'report': '举报',
        'share': '分享',
    }.get(module, module)
    
    action_cn = {
        'add': '添加',
        'check': '检查',
        'del': '删除',
        'get': '获取',
        'list': '列表',
        'manage': '管理',
        'stat': '统计',
    }.get(action, action)
    
    content = f"""// repo_adapter/src/video/{module}/{action}_port.rs  -- 🔌 视频{module_cn} - {action_cn} Port 实现
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use {trait_path};

////////

/// # [ADAPTER] - 视频{module_cn}{action_cn}
/// * `desc`: 实现视频{module_cn}的{action_cn}操作
#[derive(Debug, Default, Clone)]
pub struct {struct_name};

#[async_trait]
impl {trait_name} for {struct_name} {{
    // TODO: 实现具体的数据库操作逻辑
    // 该trait方法需要根据业务需求补全实现
}}

//////// END
"""
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f'✅ 生成: {filepath}')

# 生成所有文件
for module, actions in modules.items():
    for action in actions:
        generate_port_file(module, action)

print(f'\n📊 总共生成了 {sum(len(v) for v in modules.values())} 个 Port 文件')
