# HJKL: Code Invaders👾

A Vim motions training game written in Rust. Practice your Vim motions while defending your code from invading enemies!

![Demo](docs/demo.gif)

## How to Play

1. **Start the game** - Select "Start" from the main menu
2. **Choose your battlefield** - Enter a path to a `.rs` file, or press `Ctrl+R` to use randomly generated code
3. **Navigate with Vim motions** - Move your cursor using h/j/k/l and other Vim motions
4. **Destroy enemies** - Collide with enemies (👾) to destroy them and earn points
5. **Survive and score** - Keep destroying enemies as they move across your code!

### Keybindings

> **Note:** The game is still in active development. Not all Vim motions are implemented yet. See below for currently supported motions.

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
- `F{char}` - find previous occurrence of character (backward)
- `t{char}` - move till before next occurrence of character (forward)
- `T{char}` - move till after previous occurrence of character (backward)
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