alias l := lint


set dotenv-required
set dotenv-load := true
set dotenv-path := "server/.env"
set export :=  true
# set working-directory :="server"

DOCKER_CMD := "docker compose -f docker-compose.yaml"

lint:
    cargo fmt 
    cargo sort -w
    cargo group-imports --fix 

logs:
    {{DOCKER_CMD}} logs -f --tail='30' app

run-dev:
    {{DOCKER_CMD}} up -d
    @just logs