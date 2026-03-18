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

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* npm
  
```sh
npm install npm@latest -g
```

### Installation

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't rely on any external dependencies or services._

1. Get a free API Key at [https://example.com](https://example.com)
2. Clone the repo
   ```sh
   git clone https://github.com/github_username/repo_name.git
   ```
3. Install NPM packages
   ```sh
   npm install
   ```
4. Enter your API in `config.js`
   ```js
   const API_KEY = 'ENTER YOUR API';
   ```
5. Change git remote url to avoid accidental pushes to base project
   ```sh
   git remote set-url origin github_username/repo_name
   git remote -v # confirm the changes
   ```

## Usage

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