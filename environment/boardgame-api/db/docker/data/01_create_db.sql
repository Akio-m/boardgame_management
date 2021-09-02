CREATE USER admin WITH PASSWORD 'admin' SUPERUSER;
-- CREATE DATABASE boardgame owner admin encoding 'UTF8';
GRANT ALL PRIVILEGES ON DATABASE boardgame TO admin;

\c boardgame;
CREATE SCHEMA boardgame;

SET client_encoding = 'UTF8';