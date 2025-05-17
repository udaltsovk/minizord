CREATE TABLE sent_emails
(
    id         BINARY(16)   NOT NULL,
    receiver   VARCHAR(255) NOT NULL,
    email_type SMALLINT     NOT NULL,
    sent_at    datetime     NOT NULL,
    CONSTRAINT pk_sent_emails PRIMARY KEY (id)
);
