# random-rs
Generate random numbers from CLI

## Usage
```shell
# quick range # [<min>] <max>
$ random-rs 100
42
$ random-rs 60 50 --float 2
57.06

# range # -R <min> <max>
$ random-rs -R -- 10 -10
-4

# array # -A <min> <max> <length>
$ random-rs -A 0 100 4 --pretty
 74,  38,   8,  51

# string # -S <letters> <length>
$ random-rs -S '0,1' 8 --delimiter ''
01110100

# matrix # -M <min> <max> <columns> <rows>
$ random-rs -M 0 100 4 4 --pretty
[
 16,  48,   2,  97;
 88,  69,  13,  85;
 52,  11,  60,   3;
  6, 100,  42,  96;
]

# custom matrix # -C <min> <max> <columns> <rows> <begin> <row_begin> <column_delimiter> <row_end> <end>
$ random-rs -C 0 1 4 4 "[" " [" "," "]," " ]"
[ [0,1,0,0], [1,1,0,1], [0,0,0,1], [1,0,1,0], ]
```

## Instalation
- <img src="https://www.monitorix.org/imgs/archlinux.png" weight="20" height="20"> **Arch Linux**: from [AUR](https://aur.archlinux.org/packages/random-rs)
- **Manual**
  ```shell
  git clone https://github.com/one-d-wide/random-rs.git
  cd random-rs
  cargo build --release
  sudo cp target/release/random-rs /usr/local/bin/random
  ```