CREATE TABLE users (
	id                SERIAL PRIMARY KEY,
	password          VARCHAR(200) NOT NULL,
	username          VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);

-- Create index to make fast get queries in O(n) time
CREATE INDEX idx_users
ON users(id);
