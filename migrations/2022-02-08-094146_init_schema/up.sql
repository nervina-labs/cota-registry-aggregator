START TRANSACTION;

CREATE TABLE IF NOT EXISTS register_cota_kv_pairs (
    id bigint NOT NULL AUTO_INCREMENT,
    block_number bigint unsigned NOT NULL,
    lock_hash char(64) NOT NULL,
    created_at datetime(6) NOT NULL,
    updated_at datetime(6) NOT NULL,
    PRIMARY KEY (id),
    KEY index_register_on_block_number (block_number),
    CONSTRAINT uc_register_on_lock_hash UNIQUE (lock_hash)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

COMMIT;