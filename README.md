# `os`

## Prereqs

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install xorriso grub-common qemu grub-pc binutils-i686-linux-gnu qemu-system-x86 nasm
cargo install just
rustup component add rust-src
```

## TODO

- https://os.phil-opp.com/entering-longmode/#cpuid-check

## Resources

- Paging
  - https://wiki.osdev.org/Paging
  - https://en.wikipedia.org/wiki/Control_register
- CPUID
  - https://en.wikipedia.org/wiki/CPUID
  - https://www.felixcloutier.com/x86/cpuid
  - https://c9x.me/x86/html/file_module_x86_id_45.html

## Bye

```
cargo install bootimage
```
