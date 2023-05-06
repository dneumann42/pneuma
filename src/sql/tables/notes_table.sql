CREATE TABLE IF NOT EXISTS notes (
    id varchar(255) NOT NULL,
    title TEXT NOT NULL,
    descr TEXT NOT NULL,
    FOREIGN KEY(id) REFERENCES elements(id)
);