BEGIN TRANSACTION;
DROP TABLE IF EXISTS Movies;
CREATE TABLE IF NOT EXISTS Movies (
    movie_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    genre       TEXT NOT NULL,
    imdb_rating INTEGER NOT NULL
);

INSERT INTO Movies VALUES (1, "chalo", "action", 4.5 );
INSERT INTO Movies VALUES (2, "chale", "scifi", 4.2 );

COMMIT; 