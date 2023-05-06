CREATE TABLE IF NOT EXISTS headings (
    id varchar(255) NOT NULL,
    heading_level INTEGER NOT NULL,
    FOREIGN KEY(id) REFERENCES elements(id)
)