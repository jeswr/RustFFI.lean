# RustFFI.lean

A template for calling Rust functions in Lean using Lean's C FFI.

Based on the [Lake FFI example](https://github.com/leanprover/lake/tree/master/examples/ffi) and [Rust's FFI docs](https://doc.rust-lang.org/nomicon/ffi.html#calling-rust-code-from-c).

## Running

Build/run `Main.lean` with `lake exe ffi`

Currently results in the error

```bash
rror: > cargo build --release    # in directory .
error: stderr:
   Compiling libc v0.2.152
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.78
   Compiling unicode-ident v1.0.12
   Compiling autocfg v1.1.0
   Compiling ppv-lite86 v0.2.17
   Compiling parking_lot_core v0.9.9
   Compiling oxilangtag v0.1.3
   Compiling memchr v2.7.1
   Compiling oxiri v0.2.3-alpha.1
   Compiling syn v1.0.109
   Compiling cxxbridge-flags v1.0.115
   Compiling thiserror v1.0.56
   Compiling smallvec v1.13.1
   Compiling scopeguard v1.2.0
   Compiling lean-sys v0.0.5
   Compiling static_assertions v1.1.0
   Compiling lock_api v0.4.11
   Compiling memoffset v0.9.0
   Compiling quick-xml v0.31.0
   Compiling getrandom v0.2.12
   Compiling quote v1.0.35
   Compiling rand_core v0.6.4
   Compiling parking_lot v0.12.1
   Compiling syn v2.0.48
   Compiling rand_chacha v0.3.1
   Compiling cc v1.0.83
   Compiling rand v0.8.5
   Compiling oxrdf v0.2.0-alpha.2
   Compiling link-cplusplus v1.0.9
   Compiling cxx v1.0.115
   Compiling oxrdfxml v0.1.0-alpha.2
   Compiling oxttl v0.1.0-alpha.2
   Compiling oxrdfio v0.1.0-alpha.2
   Compiling thiserror-impl v1.0.56
   Compiling cxxbridge-macro v1.0.115
   Compiling ffi-convert-derive v0.6.1
   Compiling ffi-convert v0.6.1
   Compiling some_rust_lib v0.1.0 (/home/jesght/Documents/GitHub/RustFFI.lean)
warning: unused imports: `CReprOf`, `CStringArray`
 --> src/lib.rs:4:19
  |
4 | use ffi_convert::{CReprOf, CStringArray};
  |                   ^^^^^^^  ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `std::ffi::CStr`
 --> src/lib.rs:5:5
  |
5 | use std::ffi::CStr;
  |     ^^^^^^^^^^^^^^

warning: unused import: `std::os::raw::c_char`
 --> src/lib.rs:6:5
  |
6 | use std::os::raw::c_char;
  |     ^^^^^^^^^^^^^^^^^^^^

error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/jesght/.elan/bin:/home/jesght/.cargo/bin:/home/jesght/.elan/bin:/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:/home/jesght/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin:/snap/bin" VSLANG="1033" "cc" "-Wl,--version-script=/tmp/rustcSmVRpP/list" "-Wl,--no-undefined-version" "-m64" "/tmp/rustcSmVRpP/symbols.o" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/some_rust_lib.some_rust_lib.8afa00162ee88128-cgu.0.rcgu.o" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/some_rust_lib.2xyqpnubf8eg14vh.rcgu.o" "-Wl,--as-needed" "-L" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps" "-L" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/build/cxx-2ac55253085e8fcb/out" "-L" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/build/link-cplusplus-52158418e3dc4bd7/out" "-L" "/home/jesght/.elan/toolchains/leanprover--lean4---stable/lib" "-L" "/home/jesght/.elan/toolchains/leanprover--lean4---stable/lib/lean" "-L" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/liblean_sys-a934ba7efdc4969d.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libparking_lot-2696b8c43ba9c355.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libparking_lot_core-97003a342102a4dd.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libcfg_if-3480ae16a2eb9e66.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libsmallvec-fde2104873605d07.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/liblock_api-80a70a15c492ebb4.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libscopeguard-effe5d486d35f088.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libstatic_assertions-8e3b98cecc75fdda.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libmemoffset-60d4e505b297d921.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libffi_convert-d3602826a064a76c.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/liblibc-601a4216ccdcddd7.rlib" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libthiserror-9f11dc63178bcc03.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-90f6ddbf82de36ec.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-480c41e9d4e1f677.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-db744c0ca03eed1d.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-8ba652dc7d4b285a.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-d5f4520a9ebc0d58.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-4ad84de58f0cb463.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0d10aae2e0f38735.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-0c17cff739e6745b.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-b29e17139dde1aa8.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-1f975299829cc7bd.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-86bae3bc3079f89b.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-321b82c30dffdf5f.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-3dd8a6810a0bdfef.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-eb39a61c0c879984.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-aa769569f91c3548.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-510a192a50a983ed.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-0577018320f99037.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-193cf992125ccd4c.rlib" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-8e138eaf26ebb4a8.rlib" "-Wl,-Bdynamic" "-lm" "-ldl" "-lgmp" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "/home/jesght/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/libsome_rust_lib.so" "-Wl,--gc-sections" "-shared" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-nodefaultlibs"
  = note: /usr/bin/ld: /home/jesght/Documents/GitHub/RustFFI.lean/target/release/deps/liblean_sys-a934ba7efdc4969d.rlib(object.cpp.o): relocation R_X86_64_TPOFF32 against `_ZN4leanL21g_current_task_objectE' can not be used when making a shared object; recompile with -fPIC
          /usr/bin/ld: failed to set dynamic section sizes: bad value
          collect2: error: ld returned 1 exit status
          

warning: `some_rust_lib` (lib) generated 3 warnings
error: could not compile `some_rust_lib` (lib) due to previous error; 3 warnings emitted
error: external command `cargo` exited with code 101
```

<!-- It should print the result of `1 + 2`. -->

## Troubleshooting

Run `lake clean` and/or `cargo clean` after making changes.
