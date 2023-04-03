multiboot-header:
  nasm -f elf32 mb-header.S -o target/mb-header.o

bootloader:
  nasm -f elf32 bootloader.S -o target/bootloader.o

kernel:
  cargo build --release

image: multiboot-header bootloader kernel
  ld -n -o target/os.bin -T linker.ld -m elf_i386 target/mb-header.o target/bootloader.o target/target/release/libos.a

iso: image
  cp target/os.bin iso/os.bin
  grub-mkrescue -o os.iso iso -d /usr/lib/grub/i386-pc

boot: iso
  qemu-system-i386 -cdrom os.iso -m 64M -display sdl -cpu pentium3-v1 ; EPYC-Rome-v2

clean:
  rm -f *.o *.bin iso/os.bin os.iso
