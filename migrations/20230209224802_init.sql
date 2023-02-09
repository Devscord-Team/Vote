-- Add migration script here
CREATE TABLE Votes (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    Author TEXT NOT NULL,
    AuthorID INTEGER NOT NULL,
    Content TEXT NOT NULL,
    ServerID INTEGER NOT NULL,
    MessageID INTEGER NOT NULL,
    IsApprovedByAuthor INTEGER NOT NULL,
    ApprovedByAdminId INTEGER
)