-- Add migration script here
CREATE TABLE Quotes (
    ID INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, //AUTOINCREMENT will automatically assign the value so you dont have to do it
    Quote TEXT NOT NULL
)
