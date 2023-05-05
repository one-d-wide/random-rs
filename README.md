# random-rs
Generate random numbers from CLI

## Usage

**Quick range:** \[min\] \<max\>
```fish
$ random 100 # min = 0
42
$ random 0 1 --float 6
0.728097
```

**Range:** -R \<min\> \<max\>
```fish
$ random -R -- -100 100
-71
```

**Array:** -A \<min\> \<max\> \<length\>
```fish
$ random -A 0 100 4 --pretty
 74,  38,   8,  51
```

**String:** -S \<letters\> \<length\>
```fish
$ random -S '0,1' 8 --delimiter ''
01110100
$ random -S 'a,b,c,d' --shuffle
d, c, a, b
```


**Matrix:** -M \<min\> \<max\> \<columns\> \<rows\>
```fish
$ random -M 0 100 4 4 --pretty
[
 16,  48,   2,  97;
 88,  69,  13,  85;
 52,  11,  60,   3;
  6, 100,  42,  96;
]

$ random -M 0 100 4 4 --pretty --type diagonal
[
 11,   0,   0,   0;
  0,  83,   0,   0;
  0,   0,   3,   0;
  0,   0,   0,  98;
]
```

**Custom matrix:** -C \<min\> \<max\> \<columns\> \<rows\> \<begin\> \<row_begin\> \<delimiter\> \<row_end\> \<end\>
```fish
$ random -C 0 1 4 4 "[" " [" "," "]," " ]"
[ [0,1,0,0], [1,1,0,1], [0,0,0,1], [1,0,1,0], ]
```

## Instalation
- <img src="https://www.monitorix.org/imgs/archlinux.png" weight="20" height="20"> **Arch Linux**: From [AUR](https://aur.archlinux.org/packages/random-rs).
- **Static executable**: Check the [Releases](https://github.com/one-d-wide/random-rs/releases) page.
- **Manual**.
  ```shell
  git clone https://github.com/one-d-wide/random-rs.git
  cd random-rs
  cargo build --release
  sudo cp target/release/random-rs /usr/local/bin/random
  ```
