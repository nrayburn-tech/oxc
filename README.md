# The JavaScript Oxidation Compiler (oxc)

## Why this project?

The goal of this project is to:

* Create a blazingly fast JavaScript Compiler written in Rust.
* Provide good documentation on learning Rust and compiler techniques.

And mostly importantly, an invitation for you to come and learn Rust with me.
We will learn a lot from each other!

You can watch this project and also [follow me on twitter](https://twitter.com/boshen_c) if you don't have the time to
Rust but would like to learn things.

## Call for action

We now have a fully working parser as a baseline, it is not polished yet,
so it would be much appreciated if I can invite you and review any of the code and point out for improvements.
I welcome all nitpickings and bikesheddings.

I have crated some [discussions](https://github.com/Boshen/oxc/discussions).

## Milestone

The current objective is to improve the parser for real usage. Areas include:

* API
* Diagnostics reporting
* Performance
* Pass more conformance tests

## Conformance

The `cargo coverage` currently reports the following summary

```
Test262 Summary:
AST Parsed     : 43934/43934 (100.00%)

Babel Summary:
AST Parsed     : 2043/2057 (99.32%)

TypeScript Summary:
AST Parsed     : 4287/4861 (88.19%)
```

(The parser is failing some of the TypeScript recoverable parser tests.)

## Learning Resources

* My [small tutorial on how to write a JavaScript Parser in Rust](https://boshen.github.io/javascript-parser-in-rust/)
* [Insert your aspirational learning resources here]

## Credits

This project is inspired by the following great mentors and projects:

* [Rome Tools](https://github.com/rome/tools) - [@MichaReiser](https://github.com/MichaReiser), [@ematipico](https://github.com/ematipico)
* [Ruff](https://github.com/charliermarsh/ruff) - [@charliermarsh](https://github.com/charliermarsh)

## License

[MIT](./LICENSE)
