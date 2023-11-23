# Ankara Programming Language

## Overview

Ankara is a programming language designed to offer unique features such as `watch`, `block-level-return`, and a unified syntax for both arrays and objects. Simplify your code and enhance readability with Ankara's straightforward syntax and features.

> :warning: This project is experimental and was created to try out new syntax possibilities. It may contain several bugs, so use it at your own risk.

## Run Code

We have a sample code in `./sample/*.ank`. You can run it with the following commands.

```bash
git clone https://github.com/islandryu/Ankara.git
cargo build --release
# run ./sample/watch.ank
./run_code.sh watch
```
or

Download from [release](https://github.com/islandryu/Ankara/releases/tag/v0.0.1)

example windows
1. Download and unzip from https://github.com/islandryu/Ankara/releases/download/v0.0.1/Ankara_v0.0.1_x86_64-pc-windows-gnu.zip
2. Unzip Ankara_v0.0.1_x86_64-pc-windows-gnu.zip
3. Run command like `Ankara.exe sample.ank`

```
// sample.ank
let add = fn (a, b) {
  a + b
};

print(add(5,5));

let multiple = fn (a) {
    for(val in a) {
        val * 2
    }
};

print(multiple([1, 2, 3]));

let precedence = 1 + 2 * 3 + (4 + 5) / 3 - 10;

print(precedence)
```

## Features

### Watch

Automatically update a variable's value when its dependencies change.

### Block-Level Return

Return values directly from a block, making the code more readable and expressive.

### Unified Array and Object Syntax

Manipulate both arrays and objects using a consistent and unified syntax.

## Sample Code

### Basic Array Iteration

```ankara
let array = [1, 2, 3];

for(i in array) {
　　// 1 2 3
    print(i);
};
```

### Working with Objects

```ankara
let obj = [bar: 1, baz: 2];

// 1
print(obj["bar"]);
// 2
print(obj["baz"]);
```

### Unified Object and Array

```ankara
let objAndArray = [1, bar: 1, baz: 2];

for(i in objAndArray) {
    // 1 1 2
    print(i);
};

// 1
print(obj["bar"]);
// 2
print(obj["baz"]);
```

### Block-Level Return
If there is no semicolon at the end of the line, the value is returned at block level as block level return
```ankara
let func1 = fn () {
    let val = {
        1 + 1
    };
    return val;
};

// 2
print(func1());
```

### Watch Variables

```ankara
let x = 1;
let y = 2;

watch added = {
    x + y
};

// 3
print(added);

x = 100;

// 102
print(added);
```

## Contributing

We want as much feedback as possible.
Any issue or PR is welcome!

## License

Ankara is licensed under [MIT License](#).
