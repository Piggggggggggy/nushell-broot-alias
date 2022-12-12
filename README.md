# nushell-broot-alias
br alias for nushell

compile with cargo build --release, then add
```
def-env br [...path: string] {
# path to helper program
    let path = (~\.bin\broot-alias.exe);
    if $path =~ "cd" {
        cd ($path | parse "{cmd} {path}" | get path.0)           
    }  
}
```
to your nushell env.nu

