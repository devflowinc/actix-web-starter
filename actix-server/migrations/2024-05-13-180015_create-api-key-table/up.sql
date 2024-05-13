-- Your SQL goes here
CREATE TABLE api_keys (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL,
  name TEXT NOT NULL,
  blake3_hash TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX api_keys_user_id_idx ON api_keys(user_id);
CREATE INDEX api_keys_blake3_hash_idx ON api_keys(blake3_hash);
