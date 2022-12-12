# nushell-broot-alias
br alias for nushell

compile with `cargo build --release` - program is compiled to target\release\broot-alias.exe
```
def-env br [...path: string] {
# path to helper program
    let path = (~\.bin\broot-alias.exe);
    if $path =~ "cd" {
        cd ($path | parse "{cmd} {path}" | get path.0)           
    }  
}
```
add to your nushell env.nu

