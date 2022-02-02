# pure-std

Pure alternatives to non-pure functions in `std`!

This library is for people that like method chaining and want more of it.

Rust has plenty of types and traits (eg. `Iterator`) that leverage the ownership
model to provide methods that combine the ergonomics of functional programming
with the efficiency of imperative programming.

That being said, many functions *don't* follow this pattern, and instead offer
ways to *mutate* existing variables. For instance, if you wanted to collect the
elements of an iterator and sort them, you'd do it like this:

```rust
let mut vec: Vec<_> = very_complicated_iterator.collect();
vec.sort();
```

This is a little bit cumbersome, as we're forced to use a temporary binding for
`vec` that we don't really want. What's more, `vec` is now treated as mutable
even if we don't need it to be.

With `pure-std`, you can replace the above with this:

```rust
let vec: Vec<_> = very_complicated_iterator.collect().sorted();
```
