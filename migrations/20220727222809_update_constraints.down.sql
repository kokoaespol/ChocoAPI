ALTER TABLE users
DROP CONSTRAINT users_email_id_references_emails_id;

ALTER TABLE posts
RENAME author_id TO autor_id;
