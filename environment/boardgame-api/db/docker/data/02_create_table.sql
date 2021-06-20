\c boardgame;

CREATE TABLE boardgame.boardgame (
  name VARCHAR(1000) NOT NULL,
  name_kana VARCHAR(1000) NOT NULL,
  players_min INTEGER,
  players_max INTEGER,
  play_time_min INTEGER,
  play_time_max INTEGER,
  age INTEGER,
  manufacturer VARCHAR(1000),
  PRIMARY KEY(name)
);