# PhrenOS

A minimal Operating System. It's definitely a learning experience.
Skill issues is definitely gonna be playing a huge role in the development
process. I'm just putting it out there!!

## Build a disk image

```
cargo bootimage
```

The bootimage tool performs the following steps behind the scenes:

- It compiles the kernel to an ELF file.
- It compiles the bootloader dependency as a standalone executable.
- It links the bytes of the kernel ELF file to the bootloader.

### Booting with Qemu

```bash
cargo run
```

Under the hood this is the command being run:

```
 qemu-system-x86_64 -drive format=raw,file=target/phen_os/debug/bootimage-phren_os.bin
```

Licensed under the [MIT](LICENSE) license.
