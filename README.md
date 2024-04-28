# Building from source
## Get Rust Toolchain
```
https://www.rust-lang.org/tools/install
```

## Clone the repo
```bash
git clone https://github.com/meliosu/phystech-challenge.git
cd phystech-challenge
```

## Build
```
cargo build --release
```

## Launch
The path to binary file will be
```
./target/release/phystech-radar
```
You can optionally pass the input filename as an argument, by default it is `input.csv`
