# tasky-rs

A todo(task) manager written in Rust

## Installation

You can install tasky-rs directly using cargo:

```bash
cargo install --git https://github.com/neox1de/tasky-rs
```

## Building from source

To build the project yourself:

1. Clone the repository:
```bash
git clone https://github.com/neox1de/tasky-rs
cd tasky-rs
```

2. Build the project:
```bash
cargo build --release
```

## Usage

After instlaling with `cargo install` command, if you have add `~/.cargo/bin` to your PATH:
```bash
tasky-rs
```
otherwise, use:
```bash
cargo tasky-rs
```
if you have build the project from source, navigate to `target/release`:
```bash
cd target/release && ./tasky-rs
```
## Contributing

Contributions are welcome! Feel free to submit pull requests or create issues for bugs and feature requests.

## TODOs

Future improvements planned for tasky-rs:

- [ ] Add support for importing/exporting tasks
- [ ] Implement task categories/labels
- [ ] Add due dates for tasks
- [ ] Create task priority levels
- [ ] Add recurring tasks support
- [ ] Create desktop notifications for due tasks

## Issues

If you encounter any problems while using tasky-rs, please create an issue on the GitHub repository. I'll be happy to help!

## License

This project is open source and available under the [GNU General Public License v2.0](LICENSE).
