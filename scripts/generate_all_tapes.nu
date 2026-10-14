#!/usr/bin/env nu
# Build examples first so GIFs show results, never compilation.
def main [] {
    let root = ($env.FILE_PWD | path dirname)
    cd $root
    let b = (do { cargo build --release --examples } | complete)
    if $b.exit_code != 0 {
        print -e $b.stderr
        error make {msg: "cargo build --release --examples failed"}
    }
    for t in (glob examples/vhs/*.tape) {
        print $"vhs ($t)"
        vhs $t
    }
}
