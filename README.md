![CI](https://github.com/couchbaselabs/couchbase-shell/workflows/CI/badge.svg)

# cbsh - a new couchbase shell!

This project is a work in progress, come back later.

# Usage

If you want to run it right now, the bad news is you have to build it yourself.

Prerequisites:

 - Make sure to have a recent rust version installed (recommended: [rustup](https://rustup.rs/))
 - Install libcouchbase 3.0.0

Next, clone the repository and then run `cargo run`. The first time it will take some time since it builds all the dependencies, but you should end up in a shell. You can use `cargo run -- -h` to see all the available flags:

```
/couchbase-shell$ cargo run -- -h
   Compiling couchbase-shell v0.1.0 (/couchbase-shell)
    Finished dev [unoptimized + debuginfo] target(s) in 13.54s
     Running `target/debug/cbsh -h`
The Couchbase Shell 0.1.0
Alternative Shell and UI for Couchbase Server and Cloud

USAGE:
    cbsh [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --ui         
    -V, --version    Prints version information

OPTIONS:
    -c, --connstring <connection-string>     [default: couchbase://localhost]
    -p, --password <password>                [default: password]
    -u, --username <username>                [default: Administrator]
```

Note that if you want to spawn the ui, use the `--ui` flag.

# Supported Commands

This is heavily in flux right now, but you can try these commands (always try with `--help` if you are unsure about args and flags).

 - `query <statement>`
 - `analytics <statement>`
 - `kv-get <id>`
 - `nodes`
 - `buckets`
 - `clusters`

# Config & Multiple Clusters

While quickly connecting with command line arguments is convenient, if you manage multiple clusters it can get tedious. For this reason, the shell supports managing multiple clusters at the same time. This works by adding a `.cbshrc` to either the path where you run the binary from, or more practical, from your home directory (`~/.cbshrc`).

The format of the rc file is `toml`, and the default looks pretty much like this:

```toml
[clusters.default]
connstr = "couchbase://127.0.0.1"
username = "Administrator"
password = "password"
```

You can modify it so that it contains all the clusters you need:

```toml
[clusters.cluster1]
connstr = "couchbase://10.143.193.101"
username = "user1"
password = "pw1"

[clusters.cluster2]
connstr = "couchbase://10.143.193.102"
username = "user2"
password = "pw2"
```

You can use the `clusters` command to list them:

```
❯ clusters
───┬────────┬──────────┬────────────────────────────┬───────────────
 # │ active │ name     │ connstr                    │ username 
───┼────────┼──────────┼────────────────────────────┼───────────────
 0 │ Yes    │ cluster1 │ couchbase://10.143.193.101 │ Administrator 
 1 │ No     │ cluster2 │ couchbase://10.143.193.102 │ Administrator 
───┴────────┴──────────┴────────────────────────────┴───────────────
```

By default the first alphabetically first one (in this case `cluster1`) will be active, but you can override this on the command line with `./cbsh --cluster=cluster2` for example. This allows you to store all cluster references in one rc file and then activate the one you need.

You can switch the cluster by identifier while being in the shell `clusters --activate identifier`.

# Installing into bin

If you just want to use it and don't want to bother compiling all the time, you can use `cargo install --path .` to install it into your cargo bin path.

```
/couchbase-shell$ cargo install --path .
  Installing couchbase-shell v0.1.0 (/couchbase-shell)
    Updating git repository `https://github.com/couchbaselabs/couchbase-rs`
    Updating crates.io index
   Compiling libc v0.2.66
   Compiling proc-macro2 v1.0.8
   *** SNIP ***
   Compiling heim-process v0.0.9
   Compiling heim v0.0.9
   Compiling couchbase-shell v0.1.0 (/couchbase-shell)
    Finished release [optimized] target(s) in 8m 10s
  Installing /.cargo/bin/cbsh
   Installed package `couchbase-shell v0.1.0 (/couchbase-shell)` (executable `cbsh`)
```

Grab a quick coffee or tea since this will take some time to compile (since it compiles it in *release* mode) but then it is available in your regular path like this:

```
/couchbase-shell$ cbsh
/couchbase-shell(master)> 
```

We are going to provide standalone binaries to download later down the road once we have the CI pipelines setup.
