# `os`

## Prerequisites

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install xorriso grub-common qemu grub-pc binutils-i686-linux-gnu qemu-system-x86 nasm
cargo install just
rustup component add rust-src
```

## Resources

- x86 Instructions
  - https://www.felixcloutier.com/x86
  - https://en.wikipedia.org/wiki/X86_instruction_listings
- Paging
  - https://wiki.osdev.org/Paging
  - https://en.wikipedia.org/wiki/Control_register
- CPUID
  - https://en.wikipedia.org/wiki/CPUID
  - https://www.felixcloutier.com/x86/cpuid
  - https://c9x.me/x86/html/file_module_x86_id_45.html
  - https://en.wikichip.org/wiki/amd/cpuid
- Calling Conventions
  - https://aaronbloomfield.github.io/pdr/book/x86-32bit-ccc-chapter.pdf
  - https://en.wikipedia.org/wiki/X86_calling_conventions

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option. This means you can select the license you prefer!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
