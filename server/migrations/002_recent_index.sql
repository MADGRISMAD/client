-- La lista ordena por (destacadas, fecha, id): el índice debe cubrir las tres para no ordenar toda la tabla.
DROP INDEX IF EXISTS jobs_recent_idx;
CREATE INDEX jobs_recent_idx ON jobs (highlighted DESC, created_at DESC, id DESC);
