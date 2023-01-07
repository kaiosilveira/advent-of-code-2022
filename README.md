[![Continuous Integration](https://github.com/kaiosilveira/advent-of-code-2022/actions/workflows/rust.yml/badge.svg)](https://github.com/kaiosilveira/advent-of-code-2022/actions/workflows/rust.yml) [![Continuous Integration - NodeJS](https://github.com/kaiosilveira/advent-of-code-2022/actions/workflows/nodejs.yml/badge.svg)](https://github.com/kaiosilveira/advent-of-code-2022/actions/workflows/nodejs.yml)

# Advent of Code 2022

This repo contains code solutions for the Advent of Code challenges, with detailed challenge descriptions and step-by-step solution details, including code snippets and tests.

## Programming language choice

Most part of the solutions are implemented using Rust, as it's the language [I'm currently studying](https://github.com/kaiosilveira/the-rust-programming-language).
Rust is a language created to be fast while also providing a safe runtime. These premises are accomplished by providing a strong compile-time analysis together with a strong type system and some low-level details. Its built-in package manager and build tool, Cargo, provides all the basic tooling needed to easily get the ball rolling.

Some solutions are implemented using Javascript as a fallback for times when I didn't manage to get a solution working with Rust. These JS solutions will be ported to Rust whenever I have some time.

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
- a `README.md` with the challenge URL, description and collapsable solution blocks (with "**See solution**" as its title)
- a `sample.txt` file, containing the example described in the challenge
- a `input.txt` file, containing the actual input provided by the challenge
- a `src/lib.rs` file, containing the actual implementation and tests
- a `src/main.rs` file, containing the code that loads the input data, parses it and calls the implementation

## Solutions

The table below contains the solutions for all challenges solved so far.

| day | title                    | implementation   | part I | part II |
| --- | ------------------------ | ---------------- | ------ | ------- |
| #1  | Calorie counting         | [here](./day-01) | ✅     | ✅      |
| #2  | Rock, paper, scissors    | [here](./day-02) | ✅     | ✅      |
| #3  | Rucksack Reorganization  | [here](./day-03) | ✅     | ✅      |
| #4  | Camp Cleanup             | [here](./day-04) | ✅     | ✅      |
| #5  | Supply Stacks            | [here](./day-05) | 🚧     | 🚧      |
| #6  | Tuning Trouble           | [here](./day-06) | 🚧     | 🚧      |
| #7  | No Space Left On Device  | [here](./day-07) | 🚧     | 🚧      |
| #8  | Treetop Tree House       | [here](./day-08) | 🚧     | 🚧      |
| #9  | Rope Bridge              | [here](./day-09) | 🚧     | 🚧      |
| #10 | Cathode-Ray Tube         | [here](./day-10) | 🚧     | 🚧      |
| #11 | Monkey in the Middle     | [here](./day-11) | 🚧     | 🚧      |
| #12 | Hill Climbing Algorithm  | [here](./day-12) | 🚧     | 🚧      |
| #13 | Distress Signal          | [here](./day-13) | 🚧     | 🚧      |
| #14 | Regolith Reservoir       | [here](./day-14) | 🚧     | 🚧      |
| #15 | Beacon Exclusion Zone    | [here](./day-15) | 🚧     | ❌      |
| #16 | Proboscidea Volcanium    | [here](./day-16) | ❌     | ❌      |
| #17 | Pyroclastic Flow         | [here](./day-17) | ❌     | ❌      |
| #18 | Boiling Boulders         | [here](./day-18) | 🚧     | ❌      |
| #19 | Not Enough Minerals      | [here](./day-19) | ❌     | ❌      |
| #20 | Grove Positioning System | [here](./day-20) | ❌     | ❌      |
| #21 | Monkey Math              | [here](./day-21) | ❌     | ❌      |
| #22 | Monkey Map               | [here](./day-22) | ❌     | ❌      |
| #23 | Unstable Diffusion       | [here](./day-23) | ❌     | ❌      |
| #24 | Blizzard Basin           | [here](./day-24) | ❌     | ❌      |
| #25 | Full of Hot Air          | [here](./day-25) | ❌     | ❌      |

- ✅ = **solved and documented**
- 🚧 = **solved, but without documentation**
- ❌ = **Not solved yet**
