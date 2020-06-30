# Sipeed Longan Nano
Hello world for a `GD32VF103CBT6` RISC-V dev board.

# Prerequisites
## Rust
Recent nightly toolchain with the corresponding target:
```
rustup default nightly
rustup target add riscv32imac-unknown-none-elf
```

## Dependencies
* [RISC-V GNU toolchain](https://dl.sipeed.com/LONGAN/platformio/dl-packages/toolchain-gd32v-v9.2.0-linux.tar.gz)
* [dfu-utils](https://sourceforge.net/p/dfu-util/dfu-util/ci/master/tree/) *Important* use the latest git version, see [this issue](https://github.com/riscv-rust/longan-nano/issues/5)

Install GNU toolchain on arch linux:
```
pacman -S riscv32-elf-gdb riscv32-elf-binutils
```

# How to build & flash
Inside this repo:
```
cargo xbuild && \
riscv32-elf-objcopy -O binary target/riscv32imac-unknown-none-elf/debug/gd32vf103 firmware.bin
```

Connect the board, while holding down then `BOOT0` button shortly press the `RESET` button to enter dfu bootloader mode. Flash using dfu-util:
```
sudo dfu-util -a 0 -s 0x08000000:leave -D firmware.bin
```


# Resources & starting points
* [pcein/rust-sipeed-longan-nano](https://github.com/pcein/rust-sipeed-longan-nano) Thank you!
* [gd32v-rust/gd32vf103-example](https://github.com/gd32v-rust/gd32vf103-example) Thank you!
* [Datasheet, CPU Manual](https://github.com/SeeedDocument/Sipeed-Longan-Nano/tree/master/res)
