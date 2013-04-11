## hello_genetics

This is an implementation of "Hello world!" using a genetic algorithm. An
initial string is randomly mutated (or not mutated) until it has a higher
fitness than one of its parents (i.e. more accurately resembles the string
"Hello world!"), then breeds with another such string to produce an offspring,
which undergoes the same process.

`hello_genetics` is written in [Rust][1], Mozilla's systems language which is
actually quite cool (this is the first thing I've used it for).

[1]: http://www.rust-lang.org/

### Requirements

* Rust (`pacman -S rust` or `brew install rust`, or [follow their readme][2])

### Compiling & running

    make
    ./hello_gen

[2]: https://github.com/mozilla/rust/blob/master/README.md
