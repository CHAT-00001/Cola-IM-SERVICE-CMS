-- 用户头像与昵称历史
CREATE SCHEMA IF NOT EXISTS "cola_user";

CREATE TABLE IF NOT EXISTS "cola_user"."user_history"
(
    id BIGSERIAL PRIMARY KEY,
    uid BIGINT NOT NULL,
    history_type SMALLINT NOT NULL CHECK (history_type IN (1, 2)),
    value TEXT NOT NULL,
    status SMALLINT NOT NULL DEFAULT 1 CHECK (status IN (0, 1)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_history_uid_type_status_created
    ON "cola_user"."user_history" (uid, history_type, status, created_at DESC, id DESC);