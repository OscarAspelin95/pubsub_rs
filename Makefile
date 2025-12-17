.PHONY: all start stop

all:
	docker compose down && docker system prune -f && docker compose build &&docker compose up

start:
	docker compose up

stop:
	docker compose down
