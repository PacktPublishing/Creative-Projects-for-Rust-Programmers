All these examples have been fixed and updated to work with a more recent version of [yew](https://github.com/yewstack/yew/).
In order to run them, you need to run the steps below instead of the steps described in the book:

1. Install `wasm-pack` to compile the WASM apps: `cargo install wasm-pack`
2. Go into the app directory.
3. Run `wasm-pack build --target web --out-name wasm --out-dir ./static` to compile the application. The first run may take a long time, `wasm-pack` needs to download files from the internet to build your app.
4. Serve the app. For instance, you can install [miniserve](https://crates.io/crates/miniserve) a small webserver written in rust with `cargo install miniserve` and then serve your app with `miniserve ./static --index index.html`

## For `yclient`

This app is a bit different since it also needs as server to run correctly.

1. Go into the `person_db` folder and run `cargo run` to start the backend. This backend is similar to what you saw in the previous chapter.
2. Go into the `yclient` folder and run the steps above to compile and serve the client.
3. For the front-end, use an IP port different from 8080, that is already used to communicate with the backend. For example,use: `miniserve ./static --index index.html -p 8081`.
