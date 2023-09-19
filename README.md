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
    print(i);
};
```

### Working with Objects

```ankara
let obj = [bar: 1, baz: 2];

print(obj["bar"]);
print(obj["baz"]);
```

### Unified Object and Array

```ankara
let objAndArray = [1, bar: 1, baz: 2];

for(i in objAndArray) {
    print(i);
};
```

### Block-Level Return

```ankara
let func1 = fn () {
    let val = {
        1 + 1
    };
    return val;
};
```

### Watch Variables

```ankara
let x = 1;
let y = 2;

watch added = {
    x + y
};

//
print(added);

x = 100;

print(added);
```

## Contributing

We want as much feedback as possible.
Any issue or PR is welcome!

## License

Ankara is licensed under [MIT License](#).
