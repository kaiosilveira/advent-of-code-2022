# Advent of Code 2022

This repo contains code solutions for the Advent of Code challenges, with detailed challenge descriptions and step-by-step solution details, including code snippets and tests.

## Programming language choice

All solutions are implemented using Rust, as it's the language [I'm currently studying](https://github.com/kaiosilveira/the-rust-programming-language).
Rust is a language created to be fast while also providing a safe runtime. These premises are accomplished by providing a strong compile-time analysis together with a strong type system and some low-level details. Its built-in package manager and build tool, Cargo, provides all the basic tooling needed to easily get the ball rolling.

## Repository structure

This monorepo is structured to hold all the code for all days of the advent of code. Each directory starting with `day-` is a cargo-generated Rust project containing all the details about the challenge for that day and also all the code implemented to solve it. The file tree looks like this:

```
├── README.md
├── day-01
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── README.md
│   ├── input.txt
│   ├── sample.txt
│   └── src
│       ├── lib.rs
│       └── main.rs
.
. ... day 02 to day n-1
.
└── day-n
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── input.txt
    ├── sample.txt
    └── src
        ├── lib.rs
        └── main.rs
```

Each `day-n` project look similar, containing:

- Cargo related config files
- a README with the challenge URL, description and collapsable solution blocks (with "**See solution**" as its title)
- a `sample.txt` file, containing the example described in the challenge
- a `input.txt` file, containing the actual input provided by the challenge
- a `src/lib.rs` file, containing the actual implementation and tests
- a `src/main.rs` file, containing the code that loads the input data, parses it and calls the implementation

## Solutions

| day | description             | implementation   |
| --- | ----------------------- | ---------------- |
| #1  | Calorie counting        | [here](./day-01) |
| #2  | Rock, paper, scissors   | [here](./day-02) |
| #3  | Rucksack Reorganization | [here](./day-03) |
| #4  | Camp Cleanup            | [here](./day-04) |
