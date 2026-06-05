-- 1. 彻底清理旧的干扰项
DROP TABLE IF EXISTS "public.auth_session" CASCADE;
DROP TABLE IF EXISTS public.auth_session CASCADE;
DROP FUNCTION IF EXISTS public.update_modified_column() CASCADE;

-- 2. 规范创建全新表
CREATE TABLE public.auth_session
(
    id              BIGSERIAL PRIMARY KEY,
    send_id         VARCHAR(64)  NOT NULL,
    sync_id         VARCHAR(64)  NOT NULL UNIQUE,
    user_id         BIGINT       NOT NULL,
    access_token    VARCHAR(512) NOT NULL,
    refresh_token   VARCHAR(255) NOT NULL,
    client_id       INT          NOT NULL,
    device_id       INT          NOT NULL,
    device_name     VARCHAR(128) NOT NULL,
    last_ip         VARCHAR(45)  NOT NULL,
    platform        INT          NOT NULL,
    expired_time    BIGINT       NOT NULL,
    last_active_at  BIGINT       NOT NULL,
    status          SMALLINT     NOT NULL DEFAULT 1,
    created_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time    TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. 创建高性能复合索引
CREATE INDEX idx_auth_session_token_lookup ON public.auth_session (refresh_token) WHERE status = 1;
CREATE INDEX idx_auth_session_platform_kickout ON public.auth_session (user_id, platform) WHERE status = 1;
CREATE INDEX idx_auth_session_user_online ON public.auth_session (user_id, last_active_at DESC) WHERE status = 1;

-- 4. 创建自动化时间函数（使用完美的 plpgsql）
CREATE OR REPLACE FUNCTION public.update_modified_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_time = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 5. 绑定触发器到刚建好的表上
CREATE TRIGGER update_auth_session_modtime
    BEFORE UPDATE
    ON public.auth_session
    FOR EACH ROW
EXECUTE PROCEDURE public.update_modified_column();