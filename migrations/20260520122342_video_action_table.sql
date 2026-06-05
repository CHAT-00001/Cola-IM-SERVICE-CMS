--
-- 短视频 收藏/分享/推荐/

-- ==========================================
-- 视频动作记录表（分享 + 收藏）
-- 2026-05-20
-- ==========================================

-- 0. 创建 Schema（如果不存在）
CREATE SCHEMA IF NOT EXISTS kele_video;

-- ==========================================
-- 1. 分享记录表
-- ==========================================
CREATE TABLE IF NOT EXISTS kele_video.video_share
(
    id              BIGSERIAL PRIMARY KEY,          -- 分享记录 ID
    user_id         BIGINT      NOT NULL,           -- 用户 ID
    video_id        BIGINT      NOT NULL,           -- 视频 ID
    target_platform SMALLINT    NOT NULL DEFAULT 1, -- 目标平台
    share_code      VARCHAR(32) NOT NULL,           -- 分享码
    sync_id         UUID        NOT NULL,           -- 客户端生成的同步 ID
    sync_time       BIGINT      NOT NULL DEFAULT 0, -- 服务端同步时间（毫秒）
    create_time     BIGINT      NOT NULL            -- 客户端创建时间（毫秒）
);

-- 同步幂等
CREATE UNIQUE INDEX IF NOT EXISTS uk_video_share_sync_id
    ON kele_video.video_share (sync_id);

-- 常用查询索引
CREATE INDEX IF NOT EXISTS idx_video_share_user_id
    ON kele_video.video_share (user_id);

CREATE INDEX IF NOT EXISTS idx_video_share_video_id
    ON kele_video.video_share (video_id);

CREATE INDEX IF NOT EXISTS idx_video_share_user_video
    ON kele_video.video_share (user_id, video_id);


-- ==========================================
-- 2. 收藏记录表
-- ==========================================
CREATE TABLE IF NOT EXISTS kele_video.video_collect
(
    id          BIGSERIAL PRIMARY KEY,       -- 收藏记录 ID
    user_id     BIGINT   NOT NULL,           -- 用户 ID
    video_id    BIGINT   NOT NULL,           -- 视频 ID
    folder_id   BIGINT   NOT NULL DEFAULT 0, -- 收藏夹 ID
    channel     SMALLINT NOT NULL DEFAULT 1, -- 收藏来源
    remark      TEXT,                        -- 备注
    status      SMALLINT NOT NULL DEFAULT 0, -- 0 正常，1 已取消（软删除）
    sync_time   BIGINT   NOT NULL DEFAULT 0, -- 服务端同步时间（毫秒）
    create_time BIGINT   NOT NULL,           -- 客户端创建时间（毫秒）
    update_time BIGINT   NOT NULL            -- 更新时间（毫秒）
);

-- 防重复收藏
CREATE UNIQUE INDEX IF NOT EXISTS uk_video_collect_user_video_folder
    ON kele_video.video_collect (user_id, video_id, folder_id);

-- 常用查询索引
CREATE INDEX IF NOT EXISTS idx_video_collect_video_id
    ON kele_video.video_collect (video_id);

CREATE INDEX IF NOT EXISTS idx_video_collect_user_status_update_time
    ON kele_video.video_collect (user_id, status, update_time DESC);

CREATE INDEX IF NOT EXISTS idx_video_collect_user_folder_status
    ON kele_video.video_collect (user_id, folder_id, status);
