CREATE TABLE comments (
  id TEXT PRIMARY KEY,
  parent_id TEXT,

  message TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,

  is_deleted BOOLEAN NOT NULL DEFAULT 'f'
);