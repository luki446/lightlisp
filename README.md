
# The Light Lisp programming language

Simply the best lisp dialect (at least from these written in Rust).

## How to compile Light Lisp from source

Only requirements are [Git](https://git-scm.com/) and [Rust](https://www.rust-lang.org/)

```bash
    git clone https://github.com/luki446/lightlisp
    cd lightlisp
    cargo build --release
```

To run example

```bash
    ./target/release/llisp ./examples/simple_addition.ll
```

Or REPL mode

```bash
    ./target/release/llisp
```

## Usage

```
llisp 0.1.0
made by luki446<luki446@gmail.com>
Simple Light Lisp interpreter

USAGE:
    llisp.exe [FLAGS] [FILE]

FLAGS:
    -a, --ast        Activate Abstract Syntax Tree printing
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    File name to execute
```

## Learn Light Lisp in few (thousand) minutes

[//]: <> (TODO: Make some educational stuff)

WIP
