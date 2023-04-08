prepare:
  mkdir tmp || true

multiboot-header: prepare
  nasm -f elf32 mb-header.S -o tmp/mb-header.o

bootloader: prepare
  nasm -f elf32 bootloader.S -o tmp/bootloader.o

kernel:
  cargo build --release

image: multiboot-header bootloader kernel prepare
  ld -n -o tmp/os.bin -T linker.ld -m elf_i386 tmp/mb-header.o tmp/bootloader.o target/target/release/libos.a

iso: image
  cp tmp/os.bin iso/os.bin
  grub-mkrescue -o os.iso iso -d /usr/lib/grub/i386-pc

boot: iso
  qemu-system-i386 -cdrom os.iso -m 512M -M q35 -display sdl -cpu pentium3-v1

clean:
  rm -rf *.o *.bin iso/os.bin os.iso tmp
  cargo clean
