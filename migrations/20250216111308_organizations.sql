CREATE TABLE IF NOT EXISTS organizations (
    id INTEGER PRIMARY KEY DEFAULT generate_random_id(6),
    email VARCHAR(255) NOT NULL,
    org_code VARCHAR(255) NOT NULL,
    org_type VARCHAR(255) NOT NULL,
    org_name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    phone_code VARCHAR(20) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    owner VARCHAR(255),
    CONSTRAINT fk_owner FOREIGN KEY (owner) REFERENCES users(email) ON DELETE CASCADE
);
