<h1 align="center">📦 YARMS</h1>

Welcome to YARMS! I made YARMS because I was fed up with all the other repository management software out there. Every one I used either didn't work, was bloated, was closed source, took up a rediculous amount of server resources or had some other issues. They often had too many confusing options or features I didn't need while lacking ones I did.

Compared to the solutions I have used in the past, YARMS is a easy to deploy, lightweight, open source repository management solution.

## 🔧 Building from source

Before you can start building YARMS you must make sure you have the following software installed: [Cargo](https://rustup.rs/), [Rust](https://rustup.rs/), [NodeJS](https://nodejs.org/en/) and [Yarn](https://yarnpkg.com/). You can find installation instructions for these projects on their website. The instructions you will take for building YARMS from source depends on which environment it will be run in. The instructions for both development and production environments can be found below.

#### 🧑‍💻 Building for a development environment

**Note:** When developing, if you make changes to any frontend assets such as JavaScript, CSS or Images you must re-run `yarn build:development`.

1. Run `yarn build:development` to build the frontend code
2. Run `cargo build` to build the backend (or `cargo run` to build & run it)
3. The final built binary will be located at `./target/debug/yarms`

#### 🌐 Building for a production environment

1. Run `yarn build:production` to build the frontend code
2. Run `cargo build --release` to build the backend (or `cargo run --release` to build & run it)
3. The final built binary will be located at `./target/release/yarms`

## ⚖️ License

This project is licensed under the MIT license.
