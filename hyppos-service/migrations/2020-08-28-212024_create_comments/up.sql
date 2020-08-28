CREATE TABLE comments (
  id UUID PRIMARY KEY,
  parent_id UUID NOT NULL REFERENCES comments (id),

  user_id UUID NOT NULL REFERENCES users (id),
  project_id UUID NOT NULL REFERENCES projects (id),
  commit_id TEXT NOT NULL,
  file_id TEXT NOT NULL,
  line_no BIGINT,

  message TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,

  is_deleted BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE INDEX ON comments(parent_id);
CREATE INDEX ON comments(file_id);
