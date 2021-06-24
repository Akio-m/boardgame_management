docker run -d --name boardgame-db -p 5432:5432 --network boardgame-network --rm  -e POSTGRES_PASSWORD=boardgame akior/postgres:latest
docker run -p 8080:80 --rm --name boardgame-web boardgame-web
docker run -p 21001:21001 --network boardgame-network --rm --name boardgame-api boardgame-api