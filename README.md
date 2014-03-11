*Note: Rust has gone through a lot of changes since I wrote this, so `hello_genetics` currently doesn't compile. I haven't
kept up with the changes much, but if you feel like fixing it, feel free. Otherwise, I'll get to it when I can! Thanks!*

## hello_genetics

This is an implementation of "Hello world!" using a genetic algorithm. An
initial string is randomly mutated (or not mutated) until it has a higher
fitness than one of its parents (i.e. more accurately resembles the string
"Hello world!"). Then it breeds with another such string to produce an
offspring, which undergoes the same process.

`hello_genetics` is written in [Rust][1], Mozilla's systems language, which is
actually quite cool.

[1]: http://www.rust-lang.org/

### Requirements

* Rust (`pacman -S rust` or `brew install rust`, or [follow their readme][2])

### Compiling & running

    make
    ./hello_gen

[2]: https://github.com/mozilla/rust/blob/master/README.md
