
# Theo

A terminal-based chess engine written in Rust.

You play as white and Theo plays as black, there is no option to choose color at the moment. Moves are made in the format `e2 e4`, where `e2` is the "from" square and `e4` is the "to" square.

Theo does not currently support being used with a GUI as UCI has not been implemented yet. This may be added in the future but is not a priority.

## Installation

Clone the repo and compile with Cargo.

```bash
  git clone https://github.com/patchydev/theo.git
  cd theo/
  cargo build --release
```

You can then run with ```./target/release/theo```.
    
## Contributing

Contributions are always welcome! Feel free to make a PR.


## Authors

- [@patchydev](https://www.github.com/patchydev)


## License

[GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)

