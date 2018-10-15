# rust-gen-struct

How to get new laptop to run this project

```
cargo install llvmenv
llvmenv init
llvmenv build-entry 7.0.0
export LLVM_SYS_70_PREFIX=$HOME/.local/share/llvmenv/7.0.0
export LIBCLANG_PATH=$HOME/.local/share/llvmenv/7.0.0/lib/
```

and build

```
cd rust-gen-struct
cargo build
```

Try parse a file 

```
RUST_BACKTRACE=1 DYLD_LIBRARY_PATH=$HOME/.local/share/llvmenv/7.0.0/lib ./target/debug/rust-gen-struct /home/src/php-7.2.10/Zend/zend_modules.h _zend_module_entry
```
