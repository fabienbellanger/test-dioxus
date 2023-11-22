.PHONY: help \
	tailwind \
	web \
	desktop

.DEFAULT_GOAL=help

# Parameters
DX="dx"

help: Makefile
	@echo
	@echo "Choose a command run in "$(APP_NAME)":"
	@echo
	@sed -n 's/^##//p' $< | column -t -s ':' | sed -e 's/^/ /'
	@echo

## tailwind: Start web server
tailwind:
	npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch

## web: Start web app
web:
	$(DX) serve --platform web --port 9977

## desktop: Start desktop app
desktop:
	$(DX) serve --platform desktop

