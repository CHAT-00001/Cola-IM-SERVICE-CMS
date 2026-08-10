#!/usr/bin/env python3
# Generate stub files for all Video Ports

import os

stub_dir = "repo_adapter/src/stubs"

ports = {
    'buy': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'collect': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'comment': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'danmaku': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'dislike': ['add', 'del', 'list', 'manage', 'stat'],
    'hotlist': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'like': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'recommend': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'report': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
    'share': ['add', 'check', 'del', 'get', 'list', 'manage', 'stat'],
}

# Map of port modules to their impl struct names
trait_names = {
    'buy_add': 'BuyAddPort',
    'buy_check': 'BuyCheckPort',
    'buy_del': 'BuyDelPort',
    'buy_get': 'BuyGetPort',
    'buy_list': 'BuyListPort',
    'buy_manage': 'BuyManagePort',
    'buy_stat': 'BuyStatPort',
    # ... etc
}

os.makedirs(stub_dir, exist_ok=True)

for module, actions in ports.items():
    for action in actions:
        filename = f"{stub_dir}/{module}_{action}.rs"
        struct_name = f"{module.capitalize()}{action.capitalize()}Stub"
        trait_name = get_trait_name(module, action)
        
        content = f"""// repo_adapter/src/stubs/{module}_{action}.rs  -- 🔌 {module}/{action} Port stub
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::{module}::{action}::*;

////////

/// # [STUB] {module.capitalize()} {action.capitalize()} Port
#[derive(Debug, Default, Clone)]
pub struct {struct_name};

#[async_trait]
impl TODO for {struct_name} {{
    // TODO: Implement methods
}}

//////// END
"""
        
        with open(filename, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Generated {filename}")

print(f"Generated {sum(len(v) for v in ports.values())} stub files")
