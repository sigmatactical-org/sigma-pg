-- Keycloak uses schema keycloak in database sigma (not a separate database).
CREATE SCHEMA IF NOT EXISTS keycloak;
GRANT ALL ON SCHEMA keycloak TO sigma;
