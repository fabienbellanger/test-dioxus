# Test Dioxus

# Development
Run the following command in the root of the project to start the tailwind css compiler:
```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:
```bash
dx serve --hot-reload
dx serve --platform web --port 9977
dx serve --platform desktop
```

With `Makefile`:
```bash
make -j 2 tailwind web
make -j 2 tailwind desktop
```
