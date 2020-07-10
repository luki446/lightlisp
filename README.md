
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
Run program without arguments to enter REPL mode.
```bash
cargo run
# or
./llisp
```

## Types

In Light Lisp there are few value types

* Unsigned integer type (64-bits) ex. 123, -21
* Boolean value: true, false
* Nil value (kind of null)
* And list of these types ( 123 true -1 nil) \
_(new coming soon)_

## Syntax

Practically everything is a function (even if statement or mathematical operations). Function call construction goes like this

```
(<function_name> <function_arg_1> <function_arg_2> ...)
```

Light Lisp simplify using results of function as argument to other function.

```
(<function_name> (<function_name_2> ...) ...)
```

## Basic things

How to do basic math:

```clj
>>> (+ 1 2 3)
6
>>> (- 9 6 2)
1
>>> (* 2 2 2)
8
>>> (/ 16 4)
4
```

Comparitions and logical operations:

* equal

```clj
>>> (= 1 1 1) 
true
```

* not equal

```clj
>>> (<> 1 2 3) 
true
```

* less then (pretty cool for checking if value is in range from X yo Y)

```clj
>>> (< 1 2 3)
true
```

* less then or equal

```clj
>>> (< 1 2 3)
true
```

* greater then
  
```clj
>>> (> 3 2 1)
true
```

* greater then or equal
  
```clj
>>> (>= 3 2 1)
true
```

* logical and

```clj
>>> (&& true true)
true
```

* logical or

```clj
>>> (|| false true)
true
```

If statement have this form (for this if everything except nil and false is true)
```clj
(if <value> <true_branch> <false_branch>)
```

For example

```clj
>>> (if (< 1 5 10) 1 -1)
1
```

**_New features coming soon_**
