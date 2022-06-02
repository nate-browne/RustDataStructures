# Data Structures
## Implementing Basic Data Structures for Learning Purposes

In an effort to assist in learning Rust, this repo holds re-implementations of the
data structures I implemented over in my [CSE 12 redesign](https://github.com/nate-browne/CSE12_Redesign) but in Rust.

Each data structure more or less imitates the functionality of the C++ `stl` containers, including re-implementing the same
undefined behavior. Therefore, these structures are mostly unsuitable for actual use in any project of consequence.

If you _were_ to use these, I'd edit them so that instead of `panic!`king when something goes wrong, they smoothly handle errors
via `Result` or `Option`.

There's not much flash here, but each sub-repository will be a fully fledged Rust
project built with Cargo for ease of use.

