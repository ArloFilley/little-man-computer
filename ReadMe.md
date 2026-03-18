<h3 align="center">🏃Little Man Computer Simulator</h3>
A feature complete parsers and simulator for **Little Man Computer** assembly language

#### Table of Contents
- [About The Project](#about-the-project)
  - [Key Features:](#key-features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contributors](#contributors)
  - [Author](#author)
- [Acknowledgments](#acknowledgments)

## About The Project
A blazing fast interpreter and simulator for the world's most basic computer and compatible assembly language. Little Man Computer (or LMC as its often shortened to) is simple by design, it's a toy version of real assembly language used to teach students the basics of memory addressing, opcodes, and CPU subcomponents. LMC consists of just 256 bytes of memory, 10 instructions, and 3 addressing modes. Despite many of such simulators existing online, the ones I found are far too slow when running programs of any scale, such as prime number factor finders. This project seeks to solve this by creating a interpreter capable of running scripts and a simulator with which to run them quickly! 

### Key Features:
* Full Little Man Computer Compatible
* Interpreter capable of parsing files to be simulated
* Built with rust for incredibly fast emulation

## Getting Started
### Prerequisites
This is an example of how to list things you need to use the software and how to install them.
* [Rust](https://rustup.rs)
* [Cargo](https://rustup.rs) package manager (installed with rustup script)
  
Check Cargo is installed
```sh
cargo --help
```

Should show the following 
```sh
Rust's package manager

Usage: cargo [+toolchain] [OPTIONS] [COMMAND]
       cargo [+toolchain] [OPTIONS] -Zscript <MANIFEST_RS> [ARGS]...

Options:
  -V, --version                  Print version info and exit
      --list                     List installed commands
      --explain <CODE>           Provide a detailed explanation of a rustc error message
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring [possible values: auto, always, never]
  -C <DIRECTORY>                 Change to DIRECTORY before doing anything (nightly-only)
      --locked                   Assert that `Cargo.lock` will remain unchanged
      --offline                  Run without accessing the network
      --frozen                   Equivalent to specifying both --locked and --offline
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Commands:
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    add         Add dependencies to a manifest file
    remove      Remove dependencies from a manifest file
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary
    uninstall   Uninstall a Rust binary
    ...         See all commands with --list

See 'cargo help <command>' for more information on a specific command.
```

### Installation

1. Clone this [repo](https://github.com/ArloFilley/lmc-simulator)
   ```sh
   git clone https://github.com/github_username/repo_name.git
   ```
2. Build with Cargo
   ```sh
   cargo build --release
   ```
3. Find the executable at target/release/lmc-simulator
   * On Windows at target/release/lmc-simulator.exe

## Usage

```
Little Man Computer Simulator

Usage: lmc-simulator --input-file <INPUT_FILE>

Options:
  -i, --input-file <INPUT_FILE>  
  -h, --help                     Print help
  -V, --version                  Print version
```

- For example programs, checkout out the [Examples Folder](/examples)
- For a full list of inscructions, read [Instructions.md][/instructions.md]

## Contributing

Have something you'd like to see added, feel free to create an issue. Want to contribute, easy, just make a pull requst!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE.txt](/LISCENSE.txt) for more information.

## Contributors
### Author
Arlo Filley - [Contact Links](https://github.com/ArloFilley/ArloFilley?tab=readme-ov-file#-contact-me) 

## Acknowledgments

* [101 Computing Simulator](https://www.101computing.net/LMC/)