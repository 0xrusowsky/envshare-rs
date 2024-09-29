CREATE TABLE secrets (
    uuid UUID PRIMARY KEY,
    content TEXT,
    nonce TEXT,
    reads_left INT8 NOT NULL,
    ttl TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
