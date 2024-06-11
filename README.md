# Rust library for KolibriOS

Project uses cargo-make for building steps. You need to install cargo-binutils: 
`cargo install cargo-binutils` and llvm-tools-preview: `rustup component add llvm-tools-preview` 
to make it work. Also you need a working FASM.

Once installed building is trivial then: 
`cargo make --profile production example <example name>` 
produces a ready-to-use binary at root.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=paulcodeman/rust-kolibrios&type=Date)](https://star-history.com/#paulcodeman/rust-kolibrios&Date)
