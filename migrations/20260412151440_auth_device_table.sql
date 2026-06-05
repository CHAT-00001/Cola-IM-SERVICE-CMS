-- Add migration script here


-- =========================================================
-- 1. 物理清理：彻底炸掉旧的触发器、函数和表，防止局部残留干扰
-- =========================================================
DROP TRIGGER IF EXISTS update_auth_device_modtime ON public.auth_device;
DROP FUNCTION IF EXISTS public.update_modified_column() CASCADE;
DROP TABLE IF EXISTS public.auth_device CASCADE;

-- =========================================================
-- 2. 核心骨架：创建全新的多设备状态信任表
-- =========================================================
CREATE TABLE public.auth_device
(
    id              BIGSERIAL PRIMARY KEY,                          -- 设备记录自增 ID
    user_id         BIGINT       NOT NULL,                          -- 用户 ID
    device_sn       VARCHAR(128) NOT NULL,                          -- 设备硬件唯一序列号/UUID
    platform        INT          NOT NULL,                          -- 平台类型: 1-iOS, 2-Android, 3-Web, 4-Windows...
    device_name     VARCHAR(128) NOT NULL,                          -- 设备名称 (如: "iPhone 15 Pro")
    os_version      VARCHAR(64)  NOT NULL DEFAULT '',               -- 系统版本
    app_version     VARCHAR(64)  NOT NULL DEFAULT '',               -- App版本
    access_token    VARCHAR(512) NOT NULL,                          -- 访问令牌
    refresh_token   VARCHAR(255) NOT NULL,                          -- 刷新令牌
    last_ip         VARCHAR(45)  NOT NULL,                          -- 最近连接 IP
    is_online       SMALLINT     NOT NULL DEFAULT 0,                -- 是否在线: 1在线, 0离线
    status          SMALLINT     NOT NULL DEFAULT 1,                -- 状态: 1有效, 0注销, -1被挤掉, -2黑名单
    expired_time    BIGINT       NOT NULL,                          -- 过期时间戳
    last_active_at  BIGINT       NOT NULL,                          -- 最后的活跃时间戳
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,-- 首次绑定时间
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP -- 最后同步时间
);

-- =========================================================
-- 3. 性能起飞：挂载高性能局部复合索引
-- =========================================================
CREATE INDEX idx_auth_device_token ON public.auth_device (refresh_token) WHERE status = 1;
CREATE UNIQUE INDEX uidx_user_device_sn ON public.auth_device (user_id, device_sn);
CREATE INDEX idx_auth_device_user_platform ON public.auth_device (user_id, platform) WHERE status = 1;

-- =========================================================
-- 4. 自动化器官：创建自动化时间更新函数
-- =========================================================
CREATE OR REPLACE FUNCTION public.update_modified_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_time = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- =========================================================
-- 5. 世纪绑定：将触发器稳稳地扣在已经诞生出来的表上
-- =========================================================
CREATE TRIGGER update_auth_device_modtime
    BEFORE UPDATE ON public.auth_device
    FOR EACH ROW EXECUTE PROCEDURE public.update_modified_column();