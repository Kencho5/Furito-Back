CREATE INDEX IF NOT EXISTS idx_orgs_email ON organizations(email);
CREATE INDEX IF NOT EXISTS idx_orgs_org_code ON organizations(org_code);
CREATE INDEX IF NOT EXISTS idx_orgs_org_name ON organizations(org_name);
