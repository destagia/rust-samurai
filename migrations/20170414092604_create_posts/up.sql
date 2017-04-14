-- Your SQL goes here
CREATE TABLE posts (
	id SERIAL PRIMARY KEY,
	title VARCHAR(50) NOT NULL,
	sentence VARCHAR(2000) NOT NULL,
	account_id INTEGER NOT NULL
)