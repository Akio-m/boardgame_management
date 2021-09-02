# boardgame_management

- boardgame-api
  - Tide(Rust)
- boardgame-web
  - Vue2 + TypeScript
- environment
  - webだけ先行開発するためのmock serverがある
## local setup

### 1. boardgame-db

```
docker run -d --name boardgame-db -p 5432:5432 -e POSTGRES_PASSWORD=boardgame -e POSTGRES_USER=boardgame postgres:13.1-alpine
docker run -d --name boardgame-db -p 5432:5432 -e POSTGRES_PASSWORD=boardgame -e POSTGRES_USER=boardgame akior/postgres:latest
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

```
cargo test
cargo run
```


## build

### 1. create network
```
docker network create boardgame-network
```

### 2. start app
```
cd environment
cd boardgame-web/app/ && ./build.sh
cd -
cd boardgame-api/app/ && ./build.sh
cd -
cd boardgame-api/db/docker && ./build.sh
cd -
./start.sh
```

### 3. access app

access to [localhost:8080](localhost:8080)

### x. shutdonw app
```
cd environment
./down.sh
```


## メモ

- READMEを更新する
  - すぐに試せるようになるようなREADME

- boardgame-api
  - idがない
  - ageになっていない
  - プロパティファイルを読み込むようにする
    - "postgres://admin:admin@boardgame-db:5432/boardgame?options=-c search_path%3Dboardgame"
    - "postgres://admin:admin@localhost:5432/boardgame?options=-c search_path%3Dboardgame"

