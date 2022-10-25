# random-rs
## Simple CLI multitool
## Feaures:
- **Get quick number: ```random-rs 100 # [0..100]```**
- **Generate sequence: ```random-rs 20 60 10 # [20..60]x10```**
- **Shuflle substrings: ```random-rs -Sr 'yet,another,random,generator'```**
- **Randomize substrings: ```random-rs -S '00,01,10,11' 3 # [any]x10```**

**Check ```random-rs {-A,-R,-S} -h``` for advanced options**

## Instalation
- <img src="https://www.monitorix.org/imgs/archlinux.png" weight="20" height="20"> **Arch Linux**: in the [AUR](https://aur.archlinux.org/packages/random-rs)
- **Manual**
  ```shell
  cargo build --release
  sudo cp target/release/random-rs /usr/local/bin/random
  # enjoy
