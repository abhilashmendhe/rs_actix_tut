DB_DOCKER_CONTAINER=pg_container
DB_NAME=soccerytdb
DB_USER=root
DB_PASSWORD=secret

install:
# uncomment and indent
	cargo install cargo-edit
	cargo init
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add jsonwebtoken
	cargo add argon2
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
# SQLX-CLI
	cargo install sqlx-cli

build:
	cargo build

stop_containers:
	@echo "Stopping all docker containers"
	@if [ "$$(docker ps -q)" ]; then \
		echo "Found and stopped containers..."; \
		docker stop $$(docker ps -q); \
	else \
		echo "No active containers found..."; \
	fi

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} \
		-p 5432:5432 \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-d postgres

create_postgres_db: 
	docker exec -i ${DB_DOCKER_CONTAINER} createdb --username=${DB_USER} --owner=${DB_USER} ${DB_NAME}

start_docker_db: 
	docker start ${DB_DOCKER_CONTAINER}

run:
	cargo run

watch:
	cargo watch -q -c -w src/ -x run

init_docker: stop_containers start_docker_db

start: init_docker run