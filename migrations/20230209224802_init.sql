-- Add migration script here
CREATE TABLE VotesToApprove (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    Author TEXT NOT NULL,
    AuthorID INTEGER,
    Content TEXT NOT NULL,
    ServerID INTEGER,
    MessageID INTEGER,
    ApprovedByUser INTEGER
)