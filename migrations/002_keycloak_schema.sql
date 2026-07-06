-- Keycloak IdP: same database (sigma), dedicated schema (tables owned by Keycloak).

CREATE SCHEMA IF NOT EXISTS keycloak;
GRANT ALL ON SCHEMA keycloak TO sigma;
ALTER DEFAULT PRIVILEGES IN SCHEMA keycloak GRANT ALL ON TABLES TO sigma;
ALTER DEFAULT PRIVILEGES IN SCHEMA keycloak GRANT ALL ON SEQUENCES TO sigma;
