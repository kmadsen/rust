# Hello Rust WebAssembly

Going through the small book https://rustwasm.github.io/docs/book.
This is a "game of life" tutorial that has many parts pre-configured.

Further reading for wasm-pack https://rustwasm.github.io/docs/wasm-pack/

Further reading for wasm-bindgen https://rustwasm.github.io/docs/wasm-bindgen/

Check out the bytecodealliance projects. Specificaly wasmtime as it is a standalone
wasm runtime https://docs.wasmtime.dev/

# Installation
*This was done 2022-01-07

- Install rust with rustup https://www.rust-lang.org/tools/install
- Install nodejs and npm https://nodejs.org/en/download/package-manager/
- Install wasm https://rustwasm.github.io/wasm-pack/installer/

``` bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
$ sudo dnf install openssl-devel
$ sudo dnf module install nodejs:12
$ sudo npm install npm@latest -g
$ cargo install cargo-generate
```

Ok you should be good to go! Check out the sample wasm-game-of-life for a simple static web site.

## Issues - better to use rustup instead of dnf

When attempting to install wasm, it fails without rustup. I also didn't have rustup because I installed rustup with `dnf`.
This is a good reason to undo the `dnf install rust` (shown below)

``` bash
[09:21] $ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
wasm-pack-init: failed to find Rust installation, is rustup installed?
[09:23] $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer
warning: it looks like you have an existing installation of Rust at:
warning: /usr/bin
warning: rustup should not be installed alongside Rust. Please uninstall your existing Rust first.
warning: Otherwise you may have confusion unless you are careful with your PATH
warning: If you are sure that you want both rustup and your already installed Rust
warning: then please reply `y' or `yes' or set RUSTUP_INIT_SKIP_PATH_CHECK to yes
warning: or pass `-y' to ignore all ignorable checks.
error: cannot install while Rust is installed

Continue? (y/N) n

error: cannot install while Rust is installed
```

### Undo dnf install rust

``` bash
[09:23]$ dnf history
ID     | Command line                             | Date and time    | Action(s)      | Altered
-----------------------------------------------------------------------------------------------
     3 | install rust cargo                       | 2022-01-04 20:38 | I, U           |       8

[09:24] $ history undo 3
```

### Reinstall everything with rustup

``` bash
[09:33] $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
info: downloading installer

Welcome to Rust!
...
[09:40] $ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
info: downloading wasm-pack
info: successfully installed wasm-pack to `/home/kyle/.cargo/bin/wasm-pack`
```

## Issues - install openssl-devel



``` bash
[09:48] $ cargo install cargo-generate
...
   Compiling tempfile v3.2.0
   Compiling dialoguer v0.9.0
error: failed to run custom build command for `openssl-sys v0.9.72`

Caused by:
  process didn't exit successfully: `/tmp/cargo-installCxfQnQ/release/build/openssl-sys-8ffd7cf36cbcaf19/build-script-main` (exit status: 101)
  --- stdout
  cargo:rustc-cfg=const_fn
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
  OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
  OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_DIR
  OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=OPENSSL_STATIC
  cargo:rerun-if-env-changed=OPENSSL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  run pkg_config fail: "`\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"` did not exit successfully: exit status: 1\nerror: could not find system library 'openssl' required by the 'openssl-sys' crate\n\n--- stderr\nPackage openssl was not found in the pkg-config search path.\nPerhaps you should add the directory containing `openssl.pc'\nto the PKG_CONFIG_PATH environment variable\nPackage 'openssl', required by 'virtual:world', not found\n"

  --- stderr
  thread 'main' panicked at '

  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/sfackler/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-gnu
  openssl-sys = 0.9.72

  ', /home/kyle/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.72/build/find_normal.rs:180:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: failed to compile `cargo-generate v0.12.0`, intermediate artifacts can be found at `/tmp/cargo-installCxfQnQ`

Caused by:
  build failed
```