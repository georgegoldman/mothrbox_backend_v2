.PHONY: help docker-up docker-down docker-logs docker-clean run build test

help:
	@echo "Mothrbox Backend V2 - Available Commands:"
	@echo ""
	@echo "  make docker-up      - Start MongoDB in Docker"
	@echo "  make docker-down    - Stop MongoDB Docker container"
	@echo "  make docker-logs    - View MongoDB logs"
	@echo "  make docker-clean   - Remove MongoDB container and volumes"
	@echo "  make docker-restart - Restart MongoDB container"
	@echo "  make docker-shell   - Open MongoDB shell"
	@echo ""
	@echo "  make run            - Run the application"
	@echo "  make build          - Build the application"
	@echo "  make test           - Run tests"
	@echo "  make dev            - Run in development mode with auto-reload"
	@echo ""

# Docker commands
docker-up:
	@echo "Starting MongoDB..."
	docker-compose up -d
	@echo "MongoDB is running!"
	@echo "  - MongoDB: mongodb://localhost:27017"
	@echo "  - Mongo Express UI: http://localhost:8081 (admin/admin)"

docker-down:
	@echo "Stopping MongoDB..."
	docker-compose down

docker-logs:
	docker-compose logs -f mongodb

docker-clean:
	@echo "Removing MongoDB container and volumes..."
	docker-compose down -v
	@echo "Cleaned!"

docker-restart:
	docker-compose restart mongodb

docker-shell:
	docker exec -it mothrbox_mongodb mongosh -u admin -p password123 --authenticationDatabase admin

# Application commands
run:
	cargo run

build:
	cargo build --release

test:
	cargo test

dev:
	cargo watch -x run

# Setup commands
setup:
	@echo "Setting up project..."
	@if [ ! -f .env ]; then cp .env.example .env; echo "Created .env file"; fi
	@echo "Installing dependencies..."
	cargo build
	@echo "Setup complete! Run 'make docker-up' to start MongoDB"
