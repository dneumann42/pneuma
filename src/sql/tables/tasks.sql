CREATE TABLE IF NOT EXISTS tasks (
    id varchar(255) NOT NULL,
    content TEXT NOT NULL,
    status varchar(255) NOT NULL,
    FOREIGN KEY(id) REFERENCES elements(id) ON DELETE CASCADE
)