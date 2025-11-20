# HJKL

A Vim motions emulator game written in Rust to help practice Vim motions and have fun!

The game is currently under development, but the editor emulator is runnable (see Keybindings section for whats supported)

![Demo](docs/demo.gif)

### Keybindings

**Basic Movement**
- `h` - move left
- `j` - move down
- `k` - move up
- `l` - move right

**Word Motions**
- `w` - jump to start of next word
- `e` - jump to end of current/next word
- `b` - jump backward to start of previous word

**Character Search**
- `f{char}` - find next occurrence of character (forward)
- `t{char}` - move till before next occurrence of character (forward)
- `;` - repeat last character search
- `,` - repeat last character search in opposite direction

**Counts**
- Most motions accept a count prefix (e.g., `3j` moves down 3 lines, `5w` jumps 5 words forward)

**Commands**
- `:q` - quit the game
- `:n` - start a new round with fresh code

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

## Reference

Based on [Vim Documentation](https://vimdoc.sourceforge.net/htmldoc/motion.html).