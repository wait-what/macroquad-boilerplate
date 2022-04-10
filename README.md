# Macroquad boilerplate
## Usage

- Install dependencies

```
rustup install nightly

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

cargo install cargo-make # cargo-edit is also useful
```

- If you're on an Arch-based distro

```
sudo pacman -Sy mold upx binutils
```

> Makefile.toml will need manual changes if you don't install `mold` on linux

- Clone the repo

```
git clone https://github.com/wait-what/macroquad-boilerplate
cd macroquad-boilerplate
```

- Re-initialize git

```
rm -rf .git
git init
```

## Running

- Run the project for debugging

```
cargo make run
```

- Create a folder with releases for Windows and Linux

```
cargo make release
```

- Benchmark using mangohud

```
paru -Sy mangohud
cargo make mangohud
```

## License

This boilerpalte project is licensed under the [Unlicense license](https://unlicense.org/)
