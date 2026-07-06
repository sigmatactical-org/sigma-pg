-- Sentry monitoring service: event log and schema isolation.

CREATE SCHEMA IF NOT EXISTS sentry;

DO $$ BEGIN CREATE ROLE sentry LOGIN PASSWORD 'sigma'; EXCEPTION WHEN duplicate_object THEN NULL; END $$;

GRANT CONNECT ON DATABASE sigma TO sentry;

CREATE TABLE sentry.events (
    id TEXT PRIMARY KEY,
    service TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (
        kind IN ('status_change', 'check_failed', 'recovered', 'probe_error')
    ),
    previous_status TEXT,
    current_status TEXT NOT NULL,
    message TEXT NOT NULL,
    detail JSONB,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX sentry_events_recorded_at ON sentry.events (recorded_at DESC);
CREATE INDEX sentry_events_service ON sentry.events (service, recorded_at DESC);

REVOKE ALL ON SCHEMA sentry FROM PUBLIC;
GRANT USAGE ON SCHEMA sentry TO sentry;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA sentry TO sentry;
ALTER DEFAULT PRIVILEGES FOR ROLE sigma IN SCHEMA sentry
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO sentry;
