# Prelude
My attempt at making an IRS tax/witholings calculator in rust because i was annoyed this wasnt such a basic and trivial math/excel formula that I could blast into a simple webpage but nothing worked for me
It uses egui and other cool rust tools to render stuff,

# Build Localy
Make sure you have some of the libs to run wayland,
`apt install libwayland-egl1`
and i also just install gimp which brings in tons of default linux rendering libraries that you'll probably also need
`apt install gimp`

There is only 1 target in here, so `cargo run` should just run the main file for you target. its only tested on linux inside of windows subsystem for linux


# Build for web
* Add the wasm32 target `rustup target add wasm32-unknown-unknown`
* Check that you can build the code `cargo build --target wasm32-unknown-unknown`
* Install trunk `cargo install trunk`
* Locally
    * `trunk serve`
* Deploy
    * `trunk build --release --public-url '<base url>'`
    * copy necessary files from `dist/` to your website
