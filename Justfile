alias l := lint

set dotenv-required := true
set dotenv-load := true
set dotenv-path := ".env"
set export := true
set quiet := true


DOCKER_CMD := "docker compose -f docker-compose.yaml"

lint:
    cargo fmt 
    cargo sort -w
    cargo group-imports --fix 

logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app

run-dev:
    {{ DOCKER_CMD }} up -d
    @just logs

run-ui:
    nvm use 24
    npm run dev

test:
    DATABASE_URL=postgres://finpay:finpay@localhost:6543/finpay cargo test

[working-directory('src')]
create-module module_name:
    node ../scripts/module.js {{ module_name }}


[working-directory('scripts')]
kafka command:
    @node kafka_{{ command }}




[working-directory('scripts')]
seed command:
    @node seed_{{ command }}


