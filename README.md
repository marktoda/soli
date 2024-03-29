# Soli

Solidity version manager written in Rust.

<a href="https://github.com/marktoda/soli/stargazers"><img src="https://img.shields.io/github/stars/marktoda/soli" alt="Stars Badge"/></a>
<a href="https://github.com/marktoda/soli/pulls"><img src="https://img.shields.io/github/issues-pr/marktoda/soli" alt="Pull Requests Badge"/></a>
<a href="https://github.com/marktoda/soli/issues"><img src="https://img.shields.io/github/issues/marktoda/soli" alt="Issues Badge"/></a>
<a href="https://github.com/marktoda/soli/blob/main/LICENSE"><img src="https://img.shields.io/github/license/marktoda/soli" alt="License Badge"/></a>

## Installation

### Build from source

#### Prerequisites
* rust
  ```sh
  $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

#### Build with Cargo
```sh
$ git clone https://github.com/marktoda/soli
$ cd soli
$ cargo install --path .
```

This produces an executable `soli`


## Usage

Usage looks very similar to that of `nvm` for node.js.

### Install a Solidity version
```sh
$ soli install 0.8.4
Installing 0.8.4...
Version 0.8.4 installed.
```

Aliases:
* `soli i <version>`

### Use an installed Solidity version
```sh
$ soli use 0.8.4
Now using 0.8.4
```

Aliases:
* `soli u <version>`

### Uninstall a solidity version
```sh
$ soli uninstall 0.8.4
Version 0.8.4 uninstalled.
```

Aliases:
* `soli un <version>`

### List installed versions
```sh
$ soli list
>> 0.8.4
   0.7.1
   0.8.2
   0.8.1
   0.8.3
```

Aliases:
* `soli ls`

### List available versions
```sh
$ soli list-remote
0.8.4
0.8.3
0.8.2
0.8.1
0.8.0
0.7.6
0.7.5
0.7.4
0.7.3
0.7.2
0.7.1
0.7.0
0.6.12
0.6.11
0.6.10
0.6.9
0.6.8
0.6.7
0.6.6
0.6.5
0.5.17
0.6.4
0.6.3
0.6.2
0.6.1
0.5.16
0.6.0
0.5.15
0.5.14
```

Aliases:
* `soli lsr`
