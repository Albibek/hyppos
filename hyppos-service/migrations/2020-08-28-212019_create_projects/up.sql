CREATE TABLE projects (
  id UUID PRIMARY KEY,

  user_id UUID NOT NULL REFERENCES users (id),

  external_id BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  name TEXT NOT NULL
);

CREATE INDEX ON projects(user_id);
