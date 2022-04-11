# p2ds

**P**hysics **2D** **S**andbox. An interactive playground for exploring 2D physics simulation.

## 🛠 Building

The project uses standard [Rust](https://www.rust-lang.org/), so the easiest way to get everything working is to install [rustup](https://www.rust-lang.org/tools/install).

Once you have cargo on your system simply run
```sh
cargo build
```

## 👟 Running

You can invoke the built executable directly, or use cargo:
```sh
cargo run
```

Or, if you want to run it in the browser run
```sh
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

## 🕸 Building for web

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
- Run `wasm-pack build --target web`.
- Add built artifacts to your website.
