# ECC
## _Evil C-like Compiler_

Started learning mips at school and wanted to make this for fun

Examples can be found in the example folder and were tested using the [MARS MIPS Assembler](https://computerscience.missouristate.edu/mars-mips-simulator.htm)

## Build and Install
Since this is a rust project you'll need cargo to build and install this

Also when you build, you might get a bunch of warnings, that's okay because this isn't a serious compiler

Run these in your terminal of choice

```bash
git clone https://github.com/LD0265/ecc.git
cd ecc
cargo build -r
cargo install --path .
```

## Print Usage
```bash
ecc -h
```
or
```bash
ecc --help
```

## Usage Example
```bash
ecc hello.ec -o hello_out.asm
```