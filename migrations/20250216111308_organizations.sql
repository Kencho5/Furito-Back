CREATE TABLE IF NOT EXISTS organizations (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY (START WITH 1000),
    email VARCHAR(255) NOT NULL,
    org_code VARCHAR(255) NOT NULL,
    org_type VARCHAR(255) NOT NULL,
    org_name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    phone_code VARCHAR(20) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    owner VARCHAR(255),
    CONSTRAINT fk_owner FOREIGN KEY (owner) REFERENCES users(email) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_orgs_email ON organizations(email);
CREATE INDEX IF NOT EXISTS idx_orgs_org_code ON organizations(org_code);
CREATE INDEX IF NOT EXISTS idx_orgs_org_name ON organizations(org_name);
