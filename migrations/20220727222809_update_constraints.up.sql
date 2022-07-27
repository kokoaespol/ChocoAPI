ALTER TABLE users
ADD CONSTRAINT users_email_id_references_emails_id
    FOREIGN KEY (email_id) REFERENCES emails (id)
        ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE posts
RENAME autor_id TO author_id;
