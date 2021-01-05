# boardgame_management

- boardgame-api
- boardgame-web

## local setup

### 1. boardgame-db

```
docker run -d --name boardgame-db -p 5432:5432 -e POSTGRES_PASSWORD=boardgame -e POSTGRES_USER=boardgame postgres:13.1-alpine
```

### 2. boardgame-api

```
cargo run
```

### 3. boardgame-web

```
npm install
npm run serve
```

## development

### 1. boardgame-web

#### a. setup mock

```
environment/mockserver/json-server/run.sh
```

#### b. serve boardgame-web

```
npm run test:unit
npm run serve
```

### 2. boardgame-api

comming soon...
