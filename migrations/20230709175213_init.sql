-- Add migration script here
CREATE TABLE IF NOT EXISTS webhook_recordings (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    url text not null,
    method text not null,
    body text not null,
    headers text not null
);