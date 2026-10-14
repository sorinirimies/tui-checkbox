#!/usr/bin/env nu
# Every tape must only run `cargo run` between Hide and Show.
use std/assert

let root = ($env.FILE_PWD | path dirname | path dirname)
for t in (glob $"($root)/examples/vhs/*.tape") {
    mut hidden = false
    mut ok = true
    for l in (open $t | lines) {
        let l = ($l | str trim)
        if $l == "Hide" { $hidden = true }
        if $l == "Show" { $hidden = false }
        if ($l | str contains "cargo run") and (not $hidden) { $ok = false }
    }
    assert $ok $"visible cargo run in ($t)"
}
print "tapes ok"
