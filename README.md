# Repex

A rewrite of my `repex` module in rust.

Replaces all instances of an extension with a new extension. Helpful for converting `lua` files to `luau` quickly.

## Usage 
```sh
repex -d <dir> -o <old-ext> -n <new-ext>
```

## Example
```sh
repex -d . -o lua -n luau
```