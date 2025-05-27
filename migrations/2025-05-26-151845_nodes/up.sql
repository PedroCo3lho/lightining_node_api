-- Your SQL goes here
CREATE TABLE IF NOT EXISTS nodes (
    id SERIAL PRIMARY KEY,
    public_key VARCHAR(66) NOT NULL, -- Assuming that the public key is MAX 66 character, also Idk if it's good to strcitly type the length
    alias TEXT NOT NULL,
    capacity NUMERIC(16,8) NOT NULL DEFAULT 0,
    first_seen TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
