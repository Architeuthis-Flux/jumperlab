// Code automatically generated by `shadow-rs` (https://github.com/baoyachi/shadow-rs), do not edit.
// Author: https://www.github.com/baoyachi
// Generation time: 2024-02-13 08:47:09 -08:00



#[doc=r#"
The name of the Git branch that this project was built from.

This constant will be empty if the branch cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BRANCH :&str = r#"master"#;

#[doc=r#"
Operating system and architecture on which the project was build.
The format of this variable is always `os-arch`,
where `os` is the operating system name as returned by [`std::env::consts::OS`],
and `arch` is the computer architecture as returned by [`std::env::consts::ARCH`]."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_OS :&str = r#"macos-aarch64"#;

#[doc=r#"
The debug configuration with which the project was built.
Note that this is not the Rust channel, but either `debug` or `release`, depending on whether debug assertions were enabled in the build or not. "#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_RUST_CHANNEL :&str = r#"debug"#;

#[doc=r#"
The [target](https://doc.rust-lang.org/rustc/targets/index.html) for this build.
This is possibly distinct from the host target during build, in which case this project build was created via cross-compilation."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_TARGET :&str = r#"aarch64-apple-darwin"#;

#[doc=r#"
The architecture of the target for this build. This is the "architecture" part of the [`BUILD_TARGET`] constant."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_TARGET_ARCH :&str = r#"aarch64"#;

#[doc=r#"
The project build time, formatted in modified ISO 8601 format (`YYYY-MM-DD HH-MM ±hh-mm` where hh-mm is the offset from UTC)."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_TIME :&str = r#"2024-02-13 08:47:09 -08:00"#;

#[doc=r#"
The project build time, formatted according to [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822#section-3.3) (e.g. HTTP Headers)."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_TIME_2822 :&str = r#"Tue, 13 Feb 2024 08:47:09 -0800"#;

#[doc=r#"
The project build time, formatted according to [RFC 3339 and ISO 8601](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const BUILD_TIME_3339 :&str = r#"2024-02-13T08:47:09-08:00"#;

#[doc=r#"
The directory of the Cargo.toml manifest file of the project during build.
Note that this variable will contain a full local file system path, and will therefore contain sensitive information and not be reproducible."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const CARGO_MANIFEST_DIR :&str = r#"/Users/kevinsanto/.cargo/git/checkouts/jlctl-598af76a2a7e7934/3617ee2"#;

#[doc=r#"
The dependency tree of the project, as output by `cargo tree`.
Note that this variable may contain local file system paths for path dependencies, and may therefore contain sensitive information and not be reproducible."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const CARGO_TREE :&str = r#"
├── actix-cors v0.6.5
│   ├── actix-utils v3.0.1
│   │   ├── local-waker v0.1.4
│   │   └── pin-project-lite v0.2.13
│   ├── actix-web v4.4.0
│   │   ├── actix-codec v0.5.1
│   │   │   ├── bitflags v1.3.2
│   │   │   ├── bytes v1.5.0
│   │   │   ├── futures-core v0.3.29
│   │   │   ├── futures-sink v0.3.29
│   │   │   ├── memchr v2.6.4
│   │   │   ├── pin-project-lite v0.2.13
│   │   │   ├── tokio v1.35.1
│   │   │   │   ├── bytes v1.5.0
│   │   │   │   ├── libc v0.2.150
│   │   │   │   ├── mio v0.8.10
│   │   │   │   │   ├── libc v0.2.150
│   │   │   │   │   └── log v0.4.20
│   │   │   │   ├── parking_lot v0.12.1
│   │   │   │   │   ├── lock_api v0.4.11
│   │   │   │   │   │   └── scopeguard v1.2.0
│   │   │   │   │   │   [build-dependencies]
│   │   │   │   │   │   └── autocfg v1.1.0
│   │   │   │   │   └── parking_lot_core v0.9.9
│   │   │   │   │       ├── cfg-if v1.0.0
│   │   │   │   │       ├── libc v0.2.150
│   │   │   │   │       └── smallvec v1.11.2
│   │   │   │   ├── pin-project-lite v0.2.13
│   │   │   │   ├── signal-hook-registry v1.4.1
│   │   │   │   │   └── libc v0.2.150
│   │   │   │   └── socket2 v0.5.5
│   │   │   │       └── libc v0.2.150
│   │   │   ├── tokio-util v0.7.10
│   │   │   │   ├── bytes v1.5.0
│   │   │   │   ├── futures-core v0.3.29
│   │   │   │   ├── futures-sink v0.3.29
│   │   │   │   ├── pin-project-lite v0.2.13
│   │   │   │   ├── tokio v1.35.1 (*)
│   │   │   │   └── tracing v0.1.40
│   │   │   │       ├── log v0.4.20
│   │   │   │       ├── pin-project-lite v0.2.13
│   │   │   │       └── tracing-core v0.1.32
│   │   │   │           └── once_cell v1.19.0
│   │   │   └── tracing v0.1.40 (*)
│   │   ├── actix-http v3.4.0
│   │   │   ├── actix-codec v0.5.1 (*)
│   │   │   ├── actix-rt v2.9.0
│   │   │   │   ├── futures-core v0.3.29
│   │   │   │   └── tokio v1.35.1 (*)
│   │   │   ├── actix-service v2.0.2
│   │   │   │   ├── futures-core v0.3.29
│   │   │   │   ├── paste v1.0.14 (proc-macro)
│   │   │   │   └── pin-project-lite v0.2.13
│   │   │   ├── actix-utils v3.0.1 (*)
│   │   │   ├── ahash v0.8.6
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── getrandom v0.2.11
│   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   └── libc v0.2.150
│   │   │   │   ├── once_cell v1.19.0
│   │   │   │   └── zerocopy v0.7.32
│   │   │   │   [build-dependencies]
│   │   │   │   └── version_check v0.9.4
│   │   │   ├── base64 v0.21.5
│   │   │   ├── bitflags v2.4.1
│   │   │   ├── brotli v3.4.0
│   │   │   │   ├── alloc-no-stdlib v2.0.4
│   │   │   │   ├── alloc-stdlib v0.2.2
│   │   │   │   │   └── alloc-no-stdlib v2.0.4
│   │   │   │   └── brotli-decompressor v2.5.1
│   │   │   │       ├── alloc-no-stdlib v2.0.4
│   │   │   │       └── alloc-stdlib v0.2.2 (*)
│   │   │   ├── bytes v1.5.0
│   │   │   ├── bytestring v1.3.1
│   │   │   │   └── bytes v1.5.0
│   │   │   ├── derive_more v0.99.17 (proc-macro)
│   │   │   │   ├── convert_case v0.4.0
│   │   │   │   ├── proc-macro2 v1.0.71
│   │   │   │   │   └── unicode-ident v1.0.12
│   │   │   │   ├── quote v1.0.33
│   │   │   │   │   └── proc-macro2 v1.0.71 (*)
│   │   │   │   └── syn v1.0.109
│   │   │   │       ├── proc-macro2 v1.0.71 (*)
│   │   │   │       ├── quote v1.0.33 (*)
│   │   │   │       └── unicode-ident v1.0.12
│   │   │   │   [build-dependencies]
│   │   │   │   └── rustc_version v0.4.0
│   │   │   │       └── semver v1.0.20
│   │   │   ├── encoding_rs v0.8.33
│   │   │   │   └── cfg-if v1.0.0
│   │   │   ├── flate2 v1.0.28
│   │   │   │   ├── crc32fast v1.3.2
│   │   │   │   │   └── cfg-if v1.0.0
│   │   │   │   └── miniz_oxide v0.7.1
│   │   │   │       └── adler v1.0.2
│   │   │   ├── futures-core v0.3.29
│   │   │   ├── h2 v0.3.22
│   │   │   │   ├── bytes v1.5.0
│   │   │   │   ├── fnv v1.0.7
│   │   │   │   ├── futures-core v0.3.29
│   │   │   │   ├── futures-sink v0.3.29
│   │   │   │   ├── futures-util v0.3.29
│   │   │   │   │   ├── futures-core v0.3.29
│   │   │   │   │   ├── futures-task v0.3.29
│   │   │   │   │   ├── pin-project-lite v0.2.13
│   │   │   │   │   ├── pin-utils v0.1.0
│   │   │   │   │   └── slab v0.4.9
│   │   │   │   │       [build-dependencies]
│   │   │   │   │       └── autocfg v1.1.0
│   │   │   │   ├── http v0.2.11
│   │   │   │   │   ├── bytes v1.5.0
│   │   │   │   │   ├── fnv v1.0.7
│   │   │   │   │   └── itoa v1.0.10
│   │   │   │   ├── indexmap v2.1.0
│   │   │   │   │   ├── equivalent v1.0.1
│   │   │   │   │   └── hashbrown v0.14.3
│   │   │   │   ├── slab v0.4.9 (*)
│   │   │   │   ├── tokio v1.35.1 (*)
│   │   │   │   ├── tokio-util v0.7.10 (*)
│   │   │   │   └── tracing v0.1.40 (*)
│   │   │   ├── http v0.2.11 (*)
│   │   │   ├── httparse v1.8.0
│   │   │   ├── httpdate v1.0.3
│   │   │   ├── itoa v1.0.10
│   │   │   ├── language-tags v0.3.2
│   │   │   ├── local-channel v0.1.5
│   │   │   │   ├── futures-core v0.3.29
│   │   │   │   ├── futures-sink v0.3.29
│   │   │   │   └── local-waker v0.1.4
│   │   │   ├── mime v0.3.17
│   │   │   ├── percent-encoding v2.3.1
│   │   │   ├── pin-project-lite v0.2.13
│   │   │   ├── rand v0.8.5
│   │   │   │   ├── libc v0.2.150
│   │   │   │   ├── rand_chacha v0.3.1
│   │   │   │   │   ├── ppv-lite86 v0.2.17
│   │   │   │   │   └── rand_core v0.6.4
│   │   │   │   │       └── getrandom v0.2.11 (*)
│   │   │   │   └── rand_core v0.6.4 (*)
│   │   │   ├── sha1 v0.10.6
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── cpufeatures v0.2.11
│   │   │   │   │   └── libc v0.2.150
│   │   │   │   └── digest v0.10.7
│   │   │   │       ├── block-buffer v0.10.4
│   │   │   │       │   └── generic-array v0.14.7
│   │   │   │       │       └── typenum v1.17.0
│   │   │   │       │       [build-dependencies]
│   │   │   │       │       └── version_check v0.9.4
│   │   │   │       └── crypto-common v0.1.6
│   │   │   │           ├── generic-array v0.14.7 (*)
│   │   │   │           └── typenum v1.17.0
│   │   │   ├── smallvec v1.11.2
│   │   │   ├── tokio v1.35.1 (*)
│   │   │   ├── tokio-util v0.7.10 (*)
│   │   │   ├── tracing v0.1.40 (*)
│   │   │   └── zstd v0.12.4
│   │   │       └── zstd-safe v6.0.6
│   │   │           ├── libc v0.2.150
│   │   │           └── zstd-sys v2.0.9+zstd.1.5.5
│   │   │               [build-dependencies]
│   │   │               ├── cc v1.0.83
│   │   │               │   ├── jobserver v0.1.27
│   │   │               │   │   └── libc v0.2.150
│   │   │               │   └── libc v0.2.150
│   │   │               └── pkg-config v0.3.27
│   │   ├── actix-macros v0.2.4 (proc-macro)
│   │   │   ├── quote v1.0.33 (*)
│   │   │   └── syn v2.0.42
│   │   │       ├── proc-macro2 v1.0.71 (*)
│   │   │       ├── quote v1.0.33 (*)
│   │   │       └── unicode-ident v1.0.12
│   │   ├── actix-router v0.5.1
│   │   │   ├── bytestring v1.3.1 (*)
│   │   │   ├── http v0.2.11 (*)
│   │   │   ├── regex v1.10.2
│   │   │   │   ├── aho-corasick v1.1.2
│   │   │   │   │   └── memchr v2.6.4
│   │   │   │   ├── memchr v2.6.4
│   │   │   │   ├── regex-automata v0.4.3
│   │   │   │   │   ├── aho-corasick v1.1.2 (*)
│   │   │   │   │   ├── memchr v2.6.4
│   │   │   │   │   └── regex-syntax v0.8.2
│   │   │   │   └── regex-syntax v0.8.2
│   │   │   ├── serde v1.0.193
│   │   │   │   └── serde_derive v1.0.193 (proc-macro)
│   │   │   │       ├── proc-macro2 v1.0.71 (*)
│   │   │   │       ├── quote v1.0.33 (*)
│   │   │   │       └── syn v2.0.42 (*)
│   │   │   └── tracing v0.1.40 (*)
│   │   ├── actix-rt v2.9.0 (*)
│   │   ├── actix-server v2.3.0
│   │   │   ├── actix-rt v2.9.0 (*)
│   │   │   ├── actix-service v2.0.2 (*)
│   │   │   ├── actix-utils v3.0.1 (*)
│   │   │   ├── futures-core v0.3.29
│   │   │   ├── futures-util v0.3.29 (*)
│   │   │   ├── mio v0.8.10 (*)
│   │   │   ├── socket2 v0.5.5 (*)
│   │   │   ├── tokio v1.35.1 (*)
│   │   │   └── tracing v0.1.40 (*)
│   │   ├── actix-service v2.0.2 (*)
│   │   ├── actix-utils v3.0.1 (*)
│   │   ├── actix-web-codegen v4.2.2 (proc-macro)
│   │   │   ├── actix-router v0.5.1 (*)
│   │   │   ├── proc-macro2 v1.0.71 (*)
│   │   │   ├── quote v1.0.33 (*)
│   │   │   └── syn v2.0.42 (*)
│   │   ├── ahash v0.8.6 (*)
│   │   ├── bytes v1.5.0
│   │   ├── bytestring v1.3.1 (*)
│   │   ├── cfg-if v1.0.0
│   │   ├── cookie v0.16.2
│   │   │   ├── percent-encoding v2.3.1
│   │   │   └── time v0.3.31
│   │   │       ├── deranged v0.3.10
│   │   │       │   └── powerfmt v0.2.0
│   │   │       ├── itoa v1.0.10
│   │   │       ├── libc v0.2.150
│   │   │       ├── num_threads v0.1.6
│   │   │       │   └── libc v0.2.150
│   │   │       ├── powerfmt v0.2.0
│   │   │       ├── time-core v0.1.2
│   │   │       └── time-macros v0.2.16 (proc-macro)
│   │   │           └── time-core v0.1.2
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.4
│   │   ├── derive_more v0.99.17 (proc-macro) (*)
│   │   ├── encoding_rs v0.8.33 (*)
│   │   ├── futures-core v0.3.29
│   │   ├── futures-util v0.3.29 (*)
│   │   ├── itoa v1.0.10
│   │   ├── language-tags v0.3.2
│   │   ├── log v0.4.20
│   │   ├── mime v0.3.17
│   │   ├── once_cell v1.19.0
│   │   ├── pin-project-lite v0.2.13
│   │   ├── regex v1.10.2 (*)
│   │   ├── serde v1.0.193 (*)
│   │   ├── serde_json v1.0.108
│   │   │   ├── itoa v1.0.10
│   │   │   ├── ryu v1.0.16
│   │   │   └── serde v1.0.193 (*)
│   │   ├── serde_urlencoded v0.7.1
│   │   │   ├── form_urlencoded v1.2.1
│   │   │   │   └── percent-encoding v2.3.1
│   │   │   ├── itoa v1.0.10
│   │   │   ├── ryu v1.0.16
│   │   │   └── serde v1.0.193 (*)
│   │   ├── smallvec v1.11.2
│   │   ├── socket2 v0.5.5 (*)
│   │   ├── time v0.3.31 (*)
│   │   └── url v2.5.0
│   │       ├── form_urlencoded v1.2.1 (*)
│   │       ├── idna v0.5.0
│   │       │   ├── unicode-bidi v0.3.14
│   │       │   └── unicode-normalization v0.1.22
│   │       │       └── tinyvec v1.6.0
│   │       │           └── tinyvec_macros v0.1.1
│   │       └── percent-encoding v2.3.1
│   ├── derive_more v0.99.17 (proc-macro) (*)
│   ├── futures-util v0.3.29 (*)
│   ├── log v0.4.20
│   ├── once_cell v1.19.0
│   └── smallvec v1.11.2
├── actix-web v4.4.0 (*)
├── anyhow v1.0.76
├── clap v4.4.11
│   ├── clap_builder v4.4.11
│   │   ├── anstream v0.6.5
│   │   │   ├── anstyle v1.0.4
│   │   │   ├── anstyle-parse v0.2.3
│   │   │   │   └── utf8parse v0.2.1
│   │   │   ├── anstyle-query v1.0.2
│   │   │   ├── colorchoice v1.0.0
│   │   │   └── utf8parse v0.2.1
│   │   ├── anstyle v1.0.4
│   │   ├── clap_lex v0.6.0
│   │   └── strsim v0.10.0
│   └── clap_derive v4.4.7 (proc-macro)
│       ├── heck v0.4.1
│       ├── proc-macro2 v1.0.71 (*)
│       ├── quote v1.0.33 (*)
│       └── syn v2.0.42 (*)
├── comfy-table v7.1.0
│   ├── crossterm v0.27.0
│   │   ├── bitflags v2.4.1
│   │   ├── libc v0.2.150
│   │   └── parking_lot v0.12.1 (*)
│   ├── strum v0.25.0
│   ├── strum_macros v0.25.3 (proc-macro)
│   │   ├── heck v0.4.1
│   │   ├── proc-macro2 v1.0.71 (*)
│   │   ├── quote v1.0.33 (*)
│   │   ├── rustversion v1.0.14 (proc-macro)
│   │   └── syn v2.0.42 (*)
│   └── unicode-width v0.1.11
├── env_logger v0.10.1
│   ├── humantime v2.1.0
│   ├── is-terminal v0.4.9
│   │   └── rustix v0.38.28
│   │       ├── bitflags v2.4.1
│   │       ├── errno v0.3.8
│   │       │   └── libc v0.2.150
│   │       └── libc v0.2.150
│   ├── log v0.4.20
│   ├── regex v1.10.2 (*)
│   └── termcolor v1.4.0
├── log v0.4.20
├── mime_guess v2.0.4
│   ├── mime v0.3.17
│   └── unicase v2.7.0
│       [build-dependencies]
│       └── version_check v0.9.4
│   [build-dependencies]
│   └── unicase v2.7.0 (*)
├── nom v7.1.3
│   ├── memchr v2.6.4
│   └── minimal-lexical v0.2.1
├── serde v1.0.193 (*)
├── serde_json v1.0.108 (*)
├── serialport v4.3.1-alpha.0 (* git)
│   ├── bitflags v2.4.1
│   ├── cfg-if v1.0.0
│   ├── core-foundation-sys v0.8.6
│   ├── io-kit-sys v0.4.0
│   │   ├── core-foundation-sys v0.8.6
│   │   └── mach2 v0.4.2
│   │       └── libc v0.2.150
│   ├── mach2 v0.4.2 (*)
│   ├── nix v0.26.4
│   │   ├── bitflags v1.3.2
│   │   ├── cfg-if v1.0.0
│   │   └── libc v0.2.150
│   ├── scopeguard v1.2.0
│   └── serde v1.0.193 (*)
├── shadow-rs v0.25.0
│   ├── const_format v0.2.32
│   │   └── const_format_proc_macros v0.2.32 (proc-macro)
│   │       ├── proc-macro2 v1.0.71 (*)
│   │       ├── quote v1.0.33 (*)
│   │       └── unicode-xid v0.2.4
│   ├── git2 v0.18.1
│   │   ├── bitflags v2.4.1
│   │   ├── libc v0.2.150
│   │   ├── libgit2-sys v0.16.1+1.7.1
│   │   │   ├── libc v0.2.150
│   │   │   └── libz-sys v1.1.12
│   │   │       └── libc v0.2.150
│   │   │       [build-dependencies]
│   │   │       ├── cc v1.0.83 (*)
│   │   │       ├── pkg-config v0.3.27
│   │   │       └── vcpkg v0.2.15
│   │   │   [build-dependencies]
│   │   │   ├── cc v1.0.83 (*)
│   │   │   └── pkg-config v0.3.27
│   │   ├── log v0.4.20
│   │   └── url v2.5.0 (*)
│   ├── is_debug v1.0.1
│   ├── time v0.3.31
│   │   ├── deranged v0.3.10 (*)
│   │   ├── itoa v1.0.10
│   │   ├── libc v0.2.150
│   │   ├── num_threads v0.1.6 (*)
│   │   ├── powerfmt v0.2.0
│   │   └── time-core v0.1.2
│   └── tzdb v0.5.7
│       ├── iana-time-zone v0.1.58
│       │   └── core-foundation-sys v0.8.6
│       └── tz-rs v0.6.14
│           └── const_fn v0.4.9 (proc-macro)
└── time v0.3.31 (*)
[build-dependencies]
├── anyhow v1.0.76
└── shadow-rs v0.25.0 (*)
"#;

#[doc=r#"
The cargo version which which the project was built, as output by `cargo --version`."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const CARGO_VERSION :&str = r#"cargo 1.74.1 (ecb9851af 2023-10-18)"#;

#[doc=r#"
The author of the Git commit that this project was built from.

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_AUTHOR :&str = r#"Niklas Cathor"#;

#[doc=r#"The time of the Git commit that this project was built from.
The time is formatted in modified ISO 8601 format (`YYYY-MM-DD HH-MM ±hh-mm` where hh-mm is the offset from UTC).

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_DATE :&str = r#"2024-02-13 16:20:41 +00:00"#;

#[doc=r#"
The name of the Git branch that this project was built from.
The time is formatted according to [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822#section-3.3) (e.g. HTTP Headers).

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_DATE_2822 :&str = r#"Tue, 13 Feb 2024 16:20:41 +0000"#;

#[doc=r#"
The name of the Git branch that this project was built from.
The time is formatted according to [RFC 3339 and ISO 8601](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_DATE_3339 :&str = r#"2024-02-13T16:20:41Z"#;

#[doc=r#"
The e-mail address of the author of the Git commit that this project was built from.

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_EMAIL :&str = r#"niklas.cathor@gmx.de"#;

#[doc=r#"
The full commit hash of the Git commit that this project was built from.
An abbreviated, but not necessarily unique, version of this is [`SHORT_COMMIT`].

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const COMMIT_HASH :&str = r#"3617ee24833946af60fcff4734c9054b101c888d"#;

#[doc=r#"
Whether the Git working tree was clean at the time of project build (`true`), or not (`false`).

This constant will be `false` if the last commit cannot be determined."#]
#[allow(dead_code)]
pub const GIT_CLEAN :bool = true;

#[doc=r#"
The Git working tree status as a list of files with their status, similar to `git status`.
Each line of the list is preceded with `  * `, followed by the file name.
Files marked `(dirty)` have unstaged changes.
Files marked `(staged)` have staged changes.

This constant will be empty if the working tree status cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const GIT_STATUS_FILE :&str = r#""#;

#[doc=r#"
The name of the last Git tag on the branch that this project was built from.
As opposed to [`TAG`], this does not require the current commit to be tagged, just one of its parents.

This constant will be empty if the last tag cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const LAST_TAG :&str = r#""#;

#[doc=r#"
The project's description, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_DESCRIPTION :&str = r#""#;

#[doc=r#"
The project's full version string, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_VERSION :&str = r#"0.1.0-rc1"#;

#[doc=r#"
The project's semver major version, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_VERSION_MAJOR :&str = r#"0"#;

#[doc=r#"
The project's semver minor version, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_VERSION_MINOR :&str = r#"1"#;

#[doc=r#"
The project's semver patch version, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_VERSION_PATCH :&str = r#"0"#;

#[doc=r#"
The project's semver pre-release version, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PKG_VERSION_PRE :&str = r#"rc1"#;

#[doc=r#"
The project name, as determined by the Cargo.toml manifest."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const PROJECT_NAME :&str = r#"jlctl"#;

#[doc=r#"
The [Rustup toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) with which the project was built.
Note that as per Rustup toolchain format, this variable may or may not contain host and date information,
but it will always contain [channel](https://rust-lang.github.io/rustup/concepts/channels.html) information (stable, beta or nightly)."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const RUST_CHANNEL :&str = r#"stable-aarch64-apple-darwin"#;

#[doc=r#"
Rust version with which the project was built.
The version always uses the canonical Rust version format,
and is therefore identical to the output of the build toolchain's `rustc --version`."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const RUST_VERSION :&str = r#"rustc 1.74.1 (a28077b28 2023-12-04)"#;

#[doc=r#"
The short hash of the Git commit that this project was built from.
Note that this will always truncate [`COMMIT_HASH`] to 8 characters if necessary.
Depending on the amount of commits in your project, this may not yield a unique Git identifier
([see here for more details on hash abbreviation](https://git-scm.com/docs/git-describe#_examples)).

This constant will be empty if the last commit cannot be determined."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const SHORT_COMMIT :&str = r#"3617ee24"#;

#[doc=r#"
The name of the Git tag that this project was built from.
Note that this will be empty if there is no tag for the HEAD at the time of build."#]
#[allow(dead_code)]
#[allow(clippy::all)]
pub const TAG :&str = r#""#;


/// A long version string describing the project.
/// The version string contains the package version, branch, commit hash, build time, and build environment on separate lines.
/// This constant is suitable for printing to the user.
#[allow(dead_code)]
pub const VERSION:&str = shadow_rs::formatcp!(r#"
pkg_version:{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);

#[allow(dead_code)]
#[deprecated = "Replaced with `CLAP_LONG_VERSION`"]
pub const CLAP_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);


/// A long version string describing the project.
/// The version string contains the package version, branch, commit hash, build time, and build environment on separate lines.
/// This constant is intended to be used by clap or other CLI tools as a long version string.
#[allow(dead_code)]
pub const CLAP_LONG_VERSION:&str = shadow_rs::formatcp!(r#"{}
branch:{}
commit_hash:{}
build_time:{}
build_env:{},{}"#,PKG_VERSION, BRANCH, SHORT_COMMIT, BUILD_TIME, RUST_VERSION, RUST_CHANNEL
);

/// Prints all built-in `shadow-rs` build constants to standard output.
#[allow(dead_code)]
pub fn print_build_in() {
	println!("BRANCH:{BRANCH}\n");
	println!("BUILD_OS:{BUILD_OS}\n");
	println!("BUILD_RUST_CHANNEL:{BUILD_RUST_CHANNEL}\n");
	println!("BUILD_TARGET:{BUILD_TARGET}\n");
	println!("BUILD_TARGET_ARCH:{BUILD_TARGET_ARCH}\n");
	println!("BUILD_TIME:{BUILD_TIME}\n");
	println!("BUILD_TIME_2822:{BUILD_TIME_2822}\n");
	println!("BUILD_TIME_3339:{BUILD_TIME_3339}\n");
	println!("CARGO_MANIFEST_DIR:{CARGO_MANIFEST_DIR}\n");
	println!("CARGO_TREE:{CARGO_TREE}\n");
	println!("CARGO_VERSION:{CARGO_VERSION}\n");
	println!("COMMIT_AUTHOR:{COMMIT_AUTHOR}\n");
	println!("COMMIT_DATE:{COMMIT_DATE}\n");
	println!("COMMIT_DATE_2822:{COMMIT_DATE_2822}\n");
	println!("COMMIT_DATE_3339:{COMMIT_DATE_3339}\n");
	println!("COMMIT_EMAIL:{COMMIT_EMAIL}\n");
	println!("COMMIT_HASH:{COMMIT_HASH}\n");
	println!("GIT_CLEAN:{GIT_CLEAN}\n");
	println!("GIT_STATUS_FILE:{GIT_STATUS_FILE}\n");
	println!("LAST_TAG:{LAST_TAG}\n");
	println!("PKG_DESCRIPTION:{PKG_DESCRIPTION}\n");
	println!("PKG_VERSION:{PKG_VERSION}\n");
	println!("PKG_VERSION_MAJOR:{PKG_VERSION_MAJOR}\n");
	println!("PKG_VERSION_MINOR:{PKG_VERSION_MINOR}\n");
	println!("PKG_VERSION_PATCH:{PKG_VERSION_PATCH}\n");
	println!("PKG_VERSION_PRE:{PKG_VERSION_PRE}\n");
	println!("PROJECT_NAME:{PROJECT_NAME}\n");
	println!("RUST_CHANNEL:{RUST_CHANNEL}\n");
	println!("RUST_VERSION:{RUST_VERSION}\n");
	println!("SHORT_COMMIT:{SHORT_COMMIT}\n");
	println!("TAG:{TAG}\n");
	println!("VERSION:{VERSION}\n");
	println!("CLAP_LONG_VERSION:{CLAP_LONG_VERSION}\n");
}

