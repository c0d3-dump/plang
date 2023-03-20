# plang 

## plang is dead simple programming language for begineers.
### i have created this language for learning rust, if anyone wish to continue improvement on this language then PRs are welcomed. Thanks in advance.

### to run program

```
cargo run -- [file path with code]
```

### functions:
```rust
let x = 0
let y = 1

fn sum(t1, t2) {
    return t1 + t2
}

let z = sum(x, y)

print(z)
```

### loops
```rust
let x = 1

loop t : [1,2,3] {
    print(t)
}

loop {
    x = x + 1
    if x > 5 {
        break
    }
}

print("this is new x ", x)
```

### conditions:

```rust
let x = 1

if x == 0 {
    print("No...")
} else {
    print("Hell yeah")
}
```