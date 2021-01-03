#!/bin/bash

json=${1:-"boardgame-api.json"}

./node_modules/.bin/json-server --watch $json --routes route.json -p 21001
