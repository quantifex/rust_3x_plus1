# rust_3x_plus1
A rust based implementation of brute force 3x+1
Based on the [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture#Cycles)

## Program Flow / State Machine
```mermaid


```

## Development Environment
```shell
docker build -t 3xplus1 .
docker run -v $(pwd):/home/rust_3x_plus1 -it 3xplus1
```

## Testing
```shell
cargo test

# Test w/ code coverage
./unit_test.sh
```

## Build
```shell
cargo build --release
```

## Run
```shell
# Development
cargo run

# Release
target/release/rust_3x_plus1
```
