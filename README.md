# listd

A fast directory listing utility written in rust

## Compilation
* Just clone the repository and run ```cargo build```
* If you want to install the utility use ```cargo install --path .```

## Usage

For usage, see ```listd -h```

### Example usage
```cd $(listd -i | fzf)```

## Notes

* I mainly use it with with [fzf](https://github.com/junegunn/fzf) to create a directory fuzzy finders, since ```find``` and ```ls``` are limited and can be slow sometimes.

