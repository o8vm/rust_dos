# Rust DOS (Creating a DOS executable with Rust)

It is possible to create a DOS executable or 1st stage bootloader with Rust.  
This is a quick demo of creating COM executable for DOS.

## Building
You need a nightly Rust compiler and binutils. First you need to install the [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) and [cargo-binutils](https://github.com/rust-embedded/cargo-binutils):

```shell
cargo install cargo-xbuild 
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Then you can build the project by running:

```shell
cargo xbuild --release
```

To create a COM executable for DOS, run:

```shell
cargo objcopy -- -I elf32-i386 -O binary --binary-architecture=i386:x86 \
  target/i586-rust_dos/release/rust_dos target/i586-rust_dos/release/rust_dos.com
```

## Running
You can copy `rust_dos.com` to your DOS image.  

examples on macOS:

```shell
$ hdiutil attach path/to/freedos.img 
/dev/disk2          	FDisk_partition_scheme         	
/dev/disk2s1        	DOS_FAT_16                     	/Volumes/FREEDOS2016
$ cp target/i586-rust_dos/release/rust_dos.com /Volumes/FREEDOS2016/
```

Then, you can test it using QEMU:

```shell
qemu-system-i386 freedos.img -boot c
```

You can use the `println!` macro.  
Below is an example of HelloWorld:

![sample](https://github.com/ellbrid/rust_dos/blob/images/rust_dos_hello.png)

### Others
dpkey module steals key input processing from DOS and converts scan code to ascii code.  
about scan code: see [PS/2 Keyboard - OSDev Wiki](https://wiki.osdev.org/PS/2_Keyboard).

![sample2](https://github.com/ellbrid/rust_dos/blob/images/dpkey.gif)
