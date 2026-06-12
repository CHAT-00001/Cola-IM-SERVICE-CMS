--
-- 短视频 视频主表
-- 从 cola_data::video::entity::video::VideoEntity 自动生成
--

CREATE SCHEMA IF NOT EXISTS public;

-- ==========================================
-- 短视频主表
-- ==========================================
CREATE TABLE IF NOT EXISTS "video" (
    id              BIGSERIAL    PRIMARY KEY,              -- 视频 ID
    uid             BIGINT       NOT NULL DEFAULT 0,       -- 用户 ID
    channel_id      SMALLINT     NOT NULL DEFAULT 0,       -- 频道 ID
    title           VARCHAR(255) NOT NULL DEFAULT '',      -- 标题
    title_at_uids   BIGINT[]     DEFAULT NULL,             -- 标题@的IDs
    description     TEXT         DEFAULT NULL,             -- 描述
    desc_at_uids    BIGINT[]     DEFAULT NULL,             -- 描述@的IDs
    thumb           VARCHAR(500) NOT NULL DEFAULT '',      -- 封面
    thumb_s         VARCHAR(500) DEFAULT NULL,             -- 封面w
    thumbnail       VARCHAR(500) DEFAULT NULL,             -- 缩略图
    cover_url       VARCHAR(500) DEFAULT NULL,             -- 封面url
    href            VARCHAR(500) NOT NULL DEFAULT '',      -- 视频url
    href_w          VARCHAR(500) DEFAULT NULL,             -- 视频url w
    original_url    VARCHAR(500) DEFAULT NULL,             -- 视频原始url
    tags            TEXT[]       DEFAULT NULL,             -- 标签
    lat             DOUBLE PRECISION DEFAULT NULL,         -- 纬度
    lng             DOUBLE PRECISION DEFAULT NULL,         -- 经度
    duration        VARCHAR(20)  DEFAULT NULL,             -- 时长
    width           SMALLINT     DEFAULT NULL,             -- 帧宽度
    height          SMALLINT     DEFAULT NULL,             -- 帧高度
    fps             SMALLINT     DEFAULT NULL,             -- 帧数
    bit             SMALLINT     DEFAULT NULL,             -- 色深
    views           INTEGER      NOT NULL DEFAULT 0,       -- 浏览量
    likes           INTEGER      NOT NULL DEFAULT 0,       -- 点赞量
    dislike         INTEGER      NOT NULL DEFAULT 0,       -- 被踩数量
    collects        INTEGER      NOT NULL DEFAULT 0,       -- 收藏量
    comments        INTEGER      NOT NULL DEFAULT 0,       -- 评论数量
    danmakus        INTEGER      NOT NULL DEFAULT 0,       -- 弹幕数量
    recommends      INTEGER      NOT NULL DEFAULT 0,       -- 推荐数量
    shares          INTEGER      NOT NULL DEFAULT 0,       -- 分享数量
    is_public       BOOLEAN      DEFAULT TRUE,             -- 是否公开
    done_play_qty   INTEGER      DEFAULT 0,                -- 完成播放数量
    is_del          SMALLINT     NOT NULL DEFAULT 0,       -- 是否删除
    status          SMALLINT     NOT NULL DEFAULT 1,       -- 状态 0=下架 1=正常 2=草稿 3=冻结
    music_id        BIGINT       DEFAULT NULL,             -- 音乐id
    goods_id        BIGINT       DEFAULT NULL,             -- 商品id
    visibility_perm SMALLINT     NOT NULL DEFAULT 5,       -- 可见权限
    comment_perm    SMALLINT     NOT NULL DEFAULT 5,       -- 评论权限
    danmaku_perm    SMALLINT     NOT NULL DEFAULT 5,       -- 弹幕权限
    collect_perm    SMALLINT     NOT NULL DEFAULT 5,       -- 收藏权限
    download_perm   SMALLINT     NOT NULL DEFAULT 5,       -- 下载权限
    buy_perm        SMALLINT     NOT NULL DEFAULT 5,       -- 购买权限
    addtime         BIGINT       NOT NULL DEFAULT 0,       -- 创建时间（兼容旧版PHP）
    sync_at         BIGINT       DEFAULT NULL,             -- 同步时间
    created_at      TIMESTAMPTZ  DEFAULT NOW(),            -- 创建时间
    updated_at      TIMESTAMPTZ  DEFAULT NOW(),            -- 更新时间
    del_time        BIGINT       DEFAULT NULL,             -- 删除时间（Unix秒）
    deleted_at      TIMESTAMP    DEFAULT NULL              -- 删除时间 人类可读
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_video_uid        ON "video" (uid);
CREATE INDEX IF NOT EXISTS idx_video_status      ON "video" (status);
CREATE INDEX IF NOT EXISTS idx_video_addtime     ON "video" (addtime DESC);
CREATE INDEX IF NOT EXISTS idx_video_likes       ON "video" (likes DESC);
CREATE INDEX IF NOT EXISTS idx_video_status_addtime ON "video" (status, addtime DESC);
CREATE INDEX IF NOT EXISTS idx_video_lat_lng     ON "video" (lat, lng);

-- ==========================================
-- 插入 30 条测试数据
-- ==========================================
INSERT INTO "video" (
    uid, channel_id, title, thumb, href, duration,
    views, likes, collects, comments, shares,
    status, visibility_perm, addtime, created_at
) VALUES
(1, 1, '今天天气真好，出门溜达一圈', 'https://picsum.photos/seed/v1/400/300', 'https://example.com/video/v1.mp4', '00:15', 1520, 230, 45, 12, 8, 1, 5, 1700000000, NOW() - INTERVAL '1 hour'),
(1, 2, '自家猫咪的搞笑日常', 'https://picsum.photos/seed/v2/400/300', 'https://example.com/video/v2.mp4', '00:42', 8930, 1200, 320, 89, 56, 1, 5, 1700001000, NOW() - INTERVAL '2 hours'),
(2, 1, '早餐打卡：自制三明治', 'https://picsum.photos/seed/v3/400/300', 'https://example.com/video/v3.mp4', '00:28', 650, 98, 21, 5, 3, 1, 5, 1700002000, NOW() - INTERVAL '3 hours'),
(2, 3, '舞蹈翻跳，跳得不好轻喷', 'https://picsum.photos/seed/v4/400/300', 'https://example.com/video/v4.mp4', '01:05', 12000, 1800, 430, 150, 120, 1, 5, 1700003000, NOW() - INTERVAL '4 hours'),
(3, 1, '户外跑步5公里记录', 'https://picsum.photos/seed/v5/400/300', 'https://example.com/video/v5.mp4', '03:22', 2100, 340, 67, 23, 15, 1, 5, 1700004000, NOW() - INTERVAL '5 hours'),
(3, 2, '探店：这家咖啡馆绝了', 'https://picsum.photos/seed/v6/400/300', 'https://example.com/video/v6.mp4', '00:55', 4200, 560, 120, 38, 25, 1, 5, 1700005000, NOW() - INTERVAL '6 hours'),
(1, 1, '落日余晖，最美傍晚', 'https://picsum.photos/seed/v7/400/300', 'https://example.com/video/v7.mp4', '00:18', 3100, 480, 95, 30, 18, 1, 5, 1700006000, NOW() - INTERVAL '7 hours'),
(4, 3, '吉他弹唱《晴天》', 'https://picsum.photos/seed/v8/400/300', 'https://example.com/video/v8.mp4', '02:10', 7800, 1100, 280, 72, 45, 1, 5, 1700007000, NOW() - INTERVAL '8 hours'),
(4, 1, '健身小白的第一周记录', 'https://picsum.photos/seed/v9/400/300', 'https://example.com/video/v9.mp4', '01:30', 960, 150, 33, 10, 6, 1, 5, 1700008000, NOW() - INTERVAL '9 hours'),
(2, 2, '超简单家常菜教学', 'https://picsum.photos/seed/v10/400/300', 'https://example.com/video/v10.mp4', '02:45', 15400, 2100, 520, 180, 130, 1, 5, 1700009000, NOW() - INTERVAL '10 hours'),
(5, 1, '周末爬山游记', 'https://picsum.photos/seed/v11/400/300', 'https://example.com/video/v11.mp4', '00:38', 870, 120, 28, 8, 5, 1, 5, 1700010000, NOW() - INTERVAL '11 hours'),
(5, 3, '萌宠合集！看完心情大好', 'https://picsum.photos/seed/v12/400/300', 'https://example.com/video/v12.mp4', '01:50', 25000, 3800, 890, 320, 250, 1, 5, 1700011000, NOW() - INTERVAL '12 hours'),
(1, 2, '手工DIY：旧物改造', 'https://picsum.photos/seed/v13/400/300', 'https://example.com/video/v13.mp4', '03:15', 2300, 340, 78, 25, 16, 1, 5, 1700012000, NOW() - INTERVAL '13 hours'),
(3, 1, '海边日出直播片段', 'https://picsum.photos/seed/v14/400/300', 'https://example.com/video/v14.mp4', '00:48', 5600, 780, 160, 45, 30, 1, 5, 1700013000, NOW() - INTERVAL '14 hours'),
(4, 3, '即兴钢琴演奏', 'https://picsum.photos/seed/v15/400/300', 'https://example.com/video/v15.mp4', '02:20', 4200, 650, 140, 38, 22, 1, 5, 1700014000, NOW() - INTERVAL '15 hours'),
(2, 1, '夜跑10公里挑战', 'https://picsum.photos/seed/v16/400/300', 'https://example.com/video/v16.mp4', '04:00', 1800, 260, 55, 18, 10, 1, 5, 1700015000, NOW() - INTERVAL '16 hours'),
(5, 2, '周末Brunch教程', 'https://picsum.photos/seed/v17/400/300', 'https://example.com/video/v17.mp4', '01:20', 3900, 530, 110, 35, 20, 1, 5, 1700016000, NOW() - INTERVAL '17 hours'),
(1, 3, '街舞表演燃炸了', 'https://picsum.photos/seed/v18/400/300', 'https://example.com/video/v18.mp4', '01:45', 19000, 2800, 650, 210, 180, 1, 5, 1700017000, NOW() - INTERVAL '18 hours'),
(3, 1, '城市夜景航拍', 'https://picsum.photos/seed/v19/400/300', 'https://example.com/video/v19.mp4', '00:55', 12000, 1500, 340, 95, 60, 1, 5, 1700018000, NOW() - INTERVAL '19 hours'),
(4, 2, '新手也能做的甜点', 'https://picsum.photos/seed/v20/400/300', 'https://example.com/video/v20.mp4', '02:00', 6800, 920, 200, 60, 40, 1, 5, 1700019000, NOW() - INTERVAL '20 hours'),
(5, 1, '公园慢跑遇见小松鼠', 'https://picsum.photos/seed/v21/400/300', 'https://example.com/video/v21.mp4', '00:25', 1400, 210, 40, 15, 8, 1, 5, 1700020000, NOW() - INTERVAL '21 hours'),
(2, 3, '跟着视频学画画', 'https://picsum.photos/seed/v22/400/300', 'https://example.com/video/v22.mp4', '03:30', 3200, 480, 95, 30, 18, 1, 5, 1700021000, NOW() - INTERVAL '22 hours'),
(1, 2, '探店第二弹：隐藏在小巷的美味', 'https://picsum.photos/seed/v23/400/300', 'https://example.com/video/v23.mp4', '01:10', 5100, 720, 150, 42, 28, 1, 5, 1700022000, NOW() - INTERVAL '23 hours'),
(3, 1, '周末骑行50公里', 'https://picsum.photos/seed/v24/400/300', 'https://example.com/video/v24.mp4', '05:00', 2800, 400, 85, 22, 14, 1, 5, 1700023000, NOW() - INTERVAL '24 hours'),
(4, 3, '小提琴演奏《卡农》', 'https://picsum.photos/seed/v25/400/300', 'https://example.com/video/v25.mp4', '03:05', 11000, 1600, 380, 90, 55, 1, 5, 1700024000, NOW() - INTERVAL '25 hours'),
(5, 2, '自制珍珠奶茶教程', 'https://picsum.photos/seed/v26/400/300', 'https://example.com/video/v26.mp4', '02:30', 4500, 600, 130, 38, 24, 1, 5, 1700025000, NOW() - INTERVAL '26 hours'),
(1, 1, '晨跑看日出', 'https://picsum.photos/seed/v27/400/300', 'https://example.com/video/v27.mp4', '00:32', 1900, 290, 50, 16, 10, 1, 5, 1700026000, NOW() - INTERVAL '27 hours'),
(2, 3, '弹唱：《成都》', 'https://picsum.photos/seed/v28/400/300', 'https://example.com/video/v28.mp4', '02:50', 8200, 1300, 310, 75, 48, 1, 5, 1700027000, NOW() - INTERVAL '28 hours'),
(3, 2, '日式料理初体验', 'https://picsum.photos/seed/v29/400/300', 'https://example.com/video/v29.mp4', '03:45', 6300, 850, 180, 50, 32, 1, 5, 1700028000, NOW() - INTERVAL '29 hours'),
(4, 1, '雨中漫步，别样心情', 'https://picsum.photos/seed/v30/400/300', 'https://example.com/video/v30.mp4', '00:40', 2500, 370, 70, 20, 12, 1, 5, 1700029000, NOW() - INTERVAL '30 hours');