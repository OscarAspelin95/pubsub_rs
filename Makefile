.PHONY: all start stop build

all:
	docker compose down && docker compose build && docker compose up

start:
	docker compose up

stop:
	docker compose down

build:
	docker compose build
