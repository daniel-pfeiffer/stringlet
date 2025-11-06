# 🧵 Stringlet

A fast, cheap, compile-time constructible, `Copy`-able, kinda primitive inline string type. Stringlet length is limited
to 16, or by feature `len64`, 64 bytes. *Though the longer your stringlets, the less you should be moving and copying
them!*  No dependencies are planned, except for optional SerDe support, etc. The intention is to be no-std and
no-alloc. This might yet require feature-gating `String` interop?

<div class="warning">
This is an <b>alpha release</b>. Using it as <code>str</code> should mostly work. The design is prepared for
<code>String</code> functionality, but that needs to be implemented.
</div>

Though there is no growth (within capacity) yet, stringlets can be variable or, through a 2nd generic parameter `true`,
fixed length. In the latter case some operations like `as_str()` are cheaper, as their bounds are known at compile time.

```rust
# extern crate stringlet;
use stringlet::{Stringlet, stringlet};

let a: Stringlet<10> = "shorter".into(); // override default Stringlet capacity of 16 and don’t use all of it
let b = a;
println!("{a} == {b}? {}", a == b);      // No “value borrowed here after move” error 😇

let nothing = Stringlet::<0>::new();     // Empty and zero size
let nil = Stringlet::<5>::from_str("");  // Empty and capacity 5 and size rounded up to 8 (or 6 on small CPU)

let x = stringlet!("Hello Rust!");       // implicit length of Stringlet<11>
let y = stringlet!(14: "Hello Rust!");   // explicit capacity, can be more than length
let z = stringlet!(=14: "Hello Rust!");  // fixed length of Stringlet<14, true>
let ω = stringlet!(="Hello Rust!");      // fixed length of Stringlet<11, true>, colon optional here

const HELLO: Stringlet = stringlet!(_: "Hello Rust!"); // default capacity of Stringlet<16> derived in macro
const PETS: [Stringlet<8>; 4] = stringlet!(_: ["cat", "dog", "hamster", "piglet"]); // derive capacity for all
const PE: [Stringlet<2, true>; 4] = stringlet!(=_: ["ca", "do", "ha", "pi"]); // derive capacity for all
```

But

```text
error[E0277]: `Stringlet<99>` has excessive CAPACITY
  --> src/main.rs:99:16
   |
99 | let balloons = stringlet!(99: "Luftballons, auf ihrem…");
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ CAPACITY must be `0..=16`
   |
   = help: the trait `Config<99>` is not implemented for `Size<99>`
   = note: `Stringlet` cannot be longer than 16 bytes.
   = note: Consider activating crate feature `len64` or using `String`!
```

This is **not** your classical short string optimization (SSO) in so far as there is no overflow into an alternate
bigger storage. This is by design, as addressing two different data routes requires branching. On your hot path, branch
misprediction can be costly. Stringlet tries hard to be maximally branchless. The few `if`s and `||`s refer to
constants, so should be optimized away.

This is achieved by representing a byte buffer either as an unsigned primitive type, or, or with feature `len64`, as a
tuple of unsigned types. In how far, the latter is saf, remains to be seen. It seems to work for me. So far I have put
it neither through Miri, nor `cargo mutants`.

The length is tucked in through a no-extra-space branchless UTF-8 hack, when shorter than capacity. There is no `Option`
niche optimization yet. But, that should likewise be feasible for all stringlets shorter than physical size. I only need
to understand how to tell the compiler?

I could only come up with a rather complicated way to configure the storage for the various generic sizes. The advantage
is that `Stringlet` can only be instantiated with valid capacity. For normal use that’s all there is to it. However when
forwarding generic arguments to `Stringlet` you have to specify that whole baggage. I wish I could just hide it all
behind `<const CAPACITY: usize<0..=64>>`!

## Todo

- [ ] `StringletError` & `stringlet::Result`

- [ ] Run Miri on various architectures especially for `len64`. Who’s willing to support with exotic stuff?

- [ ] Implement mutability, `+=`, `write!()`, more traits.

- [ ] `format!()` equivalent `format_stringlet!()`

- [ ] Integrate into [string-rosetta-rs](rosetta-rs/string-rosetta-rs)

- [ ] How to implement `Cow` / `Borrow` as that seems to want to return a reference? How can a to-be-constructed `Copy`
type be owned as `String`? It is not generally possible to overlay `Stringlet` onto a `str`, as that might be too short
and not be aligned.

- [ ] Implement for popular 3rd party crates.

- [ ] Why does this not pick up the default CAPACITY of 16: `let fail = Stringlet::new();`
