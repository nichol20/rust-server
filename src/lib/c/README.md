# MathLib

A small, header-only C “mathlib” providing basic integer arithmetic (add, subtract, multiply, safe divide) as both a static and shared library. Designed to be consumable from C, C++, Rust, or any language that can call a C ABI.

---

## Features

- `add_ints(int64_t a, int64_t b)` → `a + b`  
- `sub_ints(int64_t a, int64_t b)` → `a − b`  
- `mul_ints(int64_t a, int64_t b)` → `a × b`  
- `div_ints(int64_t a, int64_t b, int64_t *result)` → safe division; returns `false` on divide-by-zero  

Each function is documented in `mathlib.h`, with full C and C++ linkage support.

## Quickstart

### 1. Build

```bash
# Build static (.a) and shared (.so) libraries:
make all
# or just:
make static
make shared
````

Build artifacts:

* `libmathlib.a` (static, PIC-friendly)
* `libmathlib.so` (shared)

### 2. Run Tests

```bash
make test
```

This will compile and run `test_mathlib`, exercising all four operations and verifying divide-by-zero handling.

---

## Usage from C / C++

```c
#include "mathlib.h"
#include <stdio.h>

int main(void) {
    int64_t x = 10, y = 3, q;
    printf("add: %lld\n", add_ints(x,y));
    printf("sub: %lld\n", sub_ints(x,y));
    printf("mul: %lld\n", mul_ints(x,y));
    if (div_ints(x,y,&q)) {
        printf("div: %lld\n", q);
    } else {
        printf("division by zero!\n");
    }
    return 0;
}
```

Compile & link (static):

```bash
gcc -I. example.c -L. -lmathlib -o example
```

---

## Usage from Rust

Copy or submodule the `src/lib/c` directory into your Cargo project. In your `build.rs`:

```rust
use std::{env, process::Command};

fn main() {
    // 1) Build the C library
    let status = Command::new("make")
        .current_dir("src/lib/c")
        .arg("static")
        .status()
        .expect("failed to run make");
    assert!(status.success());

    // 2) Tell Cargo how to link
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/src/lib/c", manifest);
    println!("cargo:rustc-link-lib=static=mathlib");

    // Re-run if C sources change
    println!("cargo:rerun-if-changed=src/lib/c/mathlib.c");
    println!("cargo:rerun-if-changed=src/lib/c/mathlib.h");
}
```

In your Rust code:

```rust
extern "C" {
    fn add_ints(a: i64, b: i64) -> i64;
    fn sub_ints(a: i64, b: i64) -> i64;
    fn mul_ints(a: i64, b: i64) -> i64;
    fn div_ints(a: i64, b: i64, result: *mut i64) -> bool;
}

fn main() {
    unsafe {
        let sum = add_ints(2, 3);
        let diff = sub_ints(5, 2);
        let prod = mul_ints(6, 7);
        let mut q = 0i64;
        let ok = div_ints(10, 2, &mut q);
        println!("2+3={} 5−2={} 6×7={} 10÷2={} (ok={})", sum, diff, prod, q, ok);
    }
}
```

---

## Makefile Targets

* `make static` — build `libmathlib.a`
* `make shared` — build `libmathlib.so`
* `make all`    — build both
* `make test`   — build & run `test_mathlib`
* `make clean`  — remove `.o`, `.a`, `.so`, and test binary
