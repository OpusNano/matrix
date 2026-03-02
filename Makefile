.PHONY: build run test clean

build:
	docker compose build

run:
	docker compose up -d

test:
	cd server && cargo test

clean:
	docker compose down -v
	cd server && cargo clean
