# Rust HTTP Server

A lightweight, multi-threaded HTTP server in Rust that demonstrates:

- **Zero-cost abstractions**: safe, ergonomic Rust code for collections, I/O, networking and threading.  
- **FFI interoperability**: call into a small C math library (`libmathlib.a`) with no runtime overhead.  
- **Custom thread-pool**: simple work-stealing thread pool (`rust_server::ThreadPool`) for handling many concurrent clients.  
- **JSON APIs & static file serving**: mix of REST endpoints and static HTML pages.  
- **Clean build artifacts**: C library built via `make` and linked through Cargo’s `build.rs`, keeping your source tree pristine.


## 🔨 Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- `gcc` compiler
- [VS Code REST Client extension](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) (optional, for `server.http`)


## 🚀 Building & Running

1. **Clone** the repo  
   ```bash
   git clone https://github.com/nichol20/rust-server.git
   cd rust-http-server-demo
   ```

2. **Build** (compiles the C code into `OUT_DIR`, then builds Rust)

   ```bash
   cargo build --release
   ```

3. **Run**

   ```bash
   cargo run --release
   ```

   The server listens on `127.0.0.1:7878`.

## 🎯 Endpoints & Usage

Most of these are covered in `server.http`—just open it in VS Code and click **“Send Request”** above each block.

### Static Files

* **`GET /`** → `static/index.html`
* **`GET /sleep`** → sleeps 5 seconds, then returns `index.html`
* **`GET /error`** → serves a non-existent file → falls back to `500.html`
* **`GET /404`** → `static/404.html`

### JSON API

* **`GET /api/hello`** → `{ "message": "Hello, world!" }`

### User Management

* **`GET /users`** → `[]` or list of users
* **`GET /users?name=<substring>&age=<n>`** → filtered list
* **`POST /users`**

  ```json
  { "name": "Alice", "age": 30 }
  ```

  → `201 Created`

### Math Operations (via C FFI)

* **`POST /math`**

  ```json
  { "operator": "+", "arg1": 10, "arg2": 5 }
  ```

  * `"+"`, `"-"`, `"*"`, `"/"`
  * Division by zero → `400 Bad Request`
  * Response example:

    ```json
    {
      "result": 50,
      "expression": "10 * 5 = 50"
    }
    ```

## 🧩 How It Works

1. **`build.rs` + Makefile**
   * `build.rs` does:

     ```rust
     Command::new("make")
         .current_dir("src/lib/c")
         .arg("static")
         …
     ```
   * Your `src/lib/c/Makefile` should produce `libmathlib.a` from `mathlib.c`.
   * Cargo link directives:

     ```rust
     println!("cargo:rustc-link-search=native={}/src/lib/c", manifest_dir);
     println!("cargo:rustc-link-lib=static=mathlib");
     ```

2. **FFI Bindings**
   * `unsafe extern "C"` declarations for `add_ints`, `sub_ints`, etc.
   * Zero-cost bridging into a hand-written C library.

3. **Thread Pool**
   * `rust_server::ThreadPool::new(4)` spins up 4 workers.
   * Each incoming connection is `execute`d as a job—no blocking the listener.

## 📝 Testing

* **Static**: open your browser at [http://127.0.0.1:7878/](http://127.0.0.1:7878/)
* **REST Client**: use `server.http` in VS Code
* **cURL**:

  ```bash
  curl -X POST http://127.0.0.1:7878/math \
       -H "Content-Type: application/json" \
       -d '{"operator":"*","arg1":6,"arg2":4}'
  ```
