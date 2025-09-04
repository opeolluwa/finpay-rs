alias l := lint


set dotenv-required
set dotenv-load := true
set dotenv-path := "server/.env"
set export :=  true
# set working-directory :="server"

DOCKER_CMD := "docker compose -f server/docker-compose.yaml"


lint:
    cargo fmt 
    cargo sort -w
    cargo group-imports --fix 
