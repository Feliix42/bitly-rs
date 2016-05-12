# bitly.rs

This is a small command line program that shortens a URL, using the [Bit.ly service](https://bitly.com).

_As one of my first projects written in the Rust programming languages the code in this repository might not be the best in terms of style. Please, feel free to open an issue or a PR to help me improve my code!_

## Getting started

To use this program, follow these steps:

1. Clone the repository with
  ```
  $ git clone git@github.com:feliix42/bitly-rs
  ```

2. Build the project.
  ```
  $ cd bitly-rs
  $ cargo build --release
  ```
  Of course you can spare the release flag if you want to help me develop this program.

3. Run it!
  ```
  $ target/release/bitly https://google.com
  Please enter your Bit.ly token:
  [enter your token here]
  http://bit.ly/1NpyVCz
  ```

## Usage
Use `$ bitly [long URL]` to shorten any URL that comes to your mind.

## Roadmap

- [x] Basic functionality
- [x] manage OAuth2 Access Token more elegant
- [x] take URL as command line argument
- [x] Error Handling (e.g. no network)
- [x] Better URL parsing
- [ ] change/remove the saved token
- [ ] Refactoring
- [ ] Documentation

## Notice
I am in no way associated with Bitly™, the only reason that this project uses Bitly as shortening service is that it has a nice API that is easy to handle. `$other_shortening_service` might also do a good job, sure!


## License

`bitly` is available under the MIT license. See the LICENSE file for more info.
