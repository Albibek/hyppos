CREATE TABLE comments (
  id UUID PRIMARY KEY,
  parent_id UUID REFERENCES comments (id),

  user_id UUID NOT NULL,
  project_id UUID NOT NULL,
  hash TEXT,
  file_id UUID NOT NULL,
  line_no bigint,

  message TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,

  is_deleted BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE INDEX ON comments(parent_id);