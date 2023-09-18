CREATE TABLE notes (
	id                SERIAL PRIMARY KEY,
	title             VARCHAR(200) NOT NULL,
	content           TEXT,
  created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- Whenever we update the table `notes`
-- The field `updated_at` will be updated automatically.
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON notes
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Create index to make fast get queries in O(n) time
CREATE INDEX idx_notes
ON notes(id);
