# HJKL

A Vim motions emulator game written in Rust to help practice Vim motions and have fun!

The game is currently under development but it's runnable with basic `h/j/k/l` motions + `w/e/b` words motions!

![Demo](docs/demo.gif)

## Installation

```bash
git clone https://github.com/jasonshyang/hjkl.git
cd hjkl
cargo build --release
```

## Usage

```bash
cargo run
```

### Keybindings

**Movement**
- `h` - left
- `j` - down
- `k` - up
- `l` - right

**Word motions**
- `w` - jump to start of next word
- `e` - jump to end of word
- `b` - jump backward to start of word

**Quit**
- `:q` - exit
- `:n` - new game

## Reference

Based on [Vim Documentation](https://vimdoc.sourceforge.net/htmldoc/motion.html).