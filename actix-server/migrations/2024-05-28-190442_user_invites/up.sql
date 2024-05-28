-- Your SQL goes here
CREATE TABLE invitations (
    id UUID PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    organization_id UUID NOT NULL,
    used BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role INT NOT NULL,
    FOREIGN KEY ("organization_id") REFERENCES "orgs"("id") ON DELETE CASCADE
);
