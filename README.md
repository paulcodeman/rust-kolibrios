# Rust Library for KolibriOS

Welcome to the Rust library for [KolibriOS](https://kolibrios.org/), a lightweight and fast open-source operating system. This library provides Rust developers with tools and abstractions to build applications for KolibriOS, leveraging Rust's safety and performance.

## Features

- **Safe and idiomatic Rust APIs** for KolibriOS system calls and functionality.
- **Easy integration** with existing KolibriOS development workflows.
- **Cargo-based build system** for streamlined compilation and binary generation.
- **Example-driven development** with sample applications to get started quickly.

## Prerequisites

To build and use this library, ensure you have the following tools installed:

- **Rust Toolchain**: Install Rust via [rustup](https://rustup.rs/).
- **cargo-binutils**: Required for binary manipulation. Install it with:
  ```bash
  cargo install cargo-binutils
  ```
- **llvm-tools-preview**: Required for additional Rust tooling. Install it with:
  ```bash
  rustup component add llvm-tools-preview
  ```
- **FASM (Flat Assembler)**: A working installation of [FASM](https://flatassembler.net/) is required for assembling KolibriOS binaries.
- **cargo-make**: Used for managing build tasks. Install it with:
  ```bash
  cargo install cargo-make
  ```

## Building the Library

Once the prerequisites are installed, building the library and examples is straightforward. To compile an example application and produce a ready-to-use binary in the project root, run:

```bash
cargo make --profile production example <example_name>
```

Replace `<example_name>` with the name of the example you want to build (e.g., `hwa`). The resulting binary will be compatible with KolibriOS.

## Examples

The repository includes several example applications to demonstrate the library's capabilities. To explore them:

1. Navigate to the `examples/` directory.
2. Check the available examples (e.g., `bf`, `hwa`).
3. Build an example using the command above.

For instance, to build and run the `hwa` example:

```bash
cargo make --profile production example hwa
```

The binary will be generated in the project root, ready to be run on KolibriOS.

## Getting Started

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/paulcodeman/rust-kolibrios.git
   cd rust-kolibrios
   ```

2. **Install Dependencies**: Follow the [Prerequisites](#prerequisites) section to set up your environment.

3. **Build an Example**: Use the `cargo make` command to build an example, as described in [Building the Library](#building-the-library).

4. **Run on KolibriOS**: Copy the generated binary to a KolibriOS environment (e.g., via a floppy disk image or emulator) and execute it.

## Contributing

Contributions are welcome! Whether you're fixing bugs, adding new features, or improving documentation, your help is appreciated. To contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please read our [Contributing Guidelines](CONTRIBUTING.md) for more details (create this file if it doesn’t exist).

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file for details.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=paulcodeman/rust-kolibrios&type=Date)](https://star-history.com/#paulcodeman/rust-kolibrios&Date)

## Support

If you encounter issues or have questions, feel free to open an issue on the [GitHub Issues page](https://github.com/paulcodeman/rust-kolibrios/issues). You can also reach out to the community via [KolibriOS forums](https://board.kolibrios.org/) or check the [Rust community](https://www.rust-lang.org/community) for additional support.
