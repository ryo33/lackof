# maybe-unimplemented

A tiny macro to generate another version of a trait with `unimplemented!()`

## Example

```rust
#[maybe_unimplemented::maybe_unimplemented]
trait Foo {
    fn foo(&self) -> u32;
    fn bar(&self) -> u32 {
        42
    }
}
```

generates, additional to the trait `Foo`, a trait `MaybeUnimplementedFoo` like
this:

```rust
trait MaybeUnimplementedFoo {
    fn foo(&self) -> u32 {
        unimplemented!()
    }
    fn bar(&self) -> u32 {
        42
    }
}

impl<T: MaybeUnimplementedFoo> Foo for T {
    fn foo(&self) -> u32 {
        MaybeUnimplementedFoo::foo(self)
    }
    fn bar(&self) -> u32 {
        MaybeUnimplementedFoo::bar(self)
    }
}
```
