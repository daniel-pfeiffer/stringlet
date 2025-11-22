# 🧵 Stringlet

A fast, cheap, compile-time constructible, `Copy`-able, kinda primitive inline string type. When storing these on the
stack, you probably want to use smaller sizes, hence the name. No dependencies are planned, except for optional SerDe
support, etc. The intention is to be no-std and no-alloc – which still requires feature-gating `String` interop.

<div class="warning">
This is an <b>alpha release</b>. Using it as <code>str</code> should mostly work. The design is prepared for
<code>String</code> functionality, but that needs to be implemented.
</div>

In my casual benchmarking it beats all other string kinds and crates nicely to spectacularly on various tests. There are
four flavors of mostly the same code. They differ in length handling, which shows only in some operations, like
`len()`, `as_ref()`, and `as_str()`:

- **`Stringlet`, `stringlet!(…)`**: This is fixed size, i.e. bounds for array access are compiled in, hence fast.

- **`VarStringlet`, `stringlet!(var …)`, `stringlet!(v …)`**: This adds one byte for the length – still pretty fast.
  Speed differs for some content processing, where SIMD gives an advantage for multiples of some power of 2, e.g.
  `VarStringlet<32>`. While for copying the advantage can be at one less, e.g. `VarStringlet<31>`. Length must be
  `0..=255`.

- **`TrimStringlet`, `stringlet!(trim …)`, `stringlet!(t …)`**: This can optionally trim one last byte, useful for codes
  with minimal length variation like [ISO 639](https://www.iso.org/iso-639-language-code). This is achieved by tagging
  an unused last byte with a UTF-8 niche. The length gets calculated branchlessly with very few ops.

- **`SlimStringlet`, `stringlet!(slim …)`, `stringlet!(s …)`**: This uses the same UTF-8 niche, but fully: It projects
  the length into 6 bits of the last byte, when content is less than full size. Length must be `0..=64`. Though it is
  done branchlessly, there are a few more ops for length calculation. Hence this is the slowest, albeit by a small
  margin. I’m still racking my brain for how to do it with less ops. Any bit hackers, welcome on board!

N.B.: Variable size `VarStringlet` seems a competitor to [`fixedstr::str`](https://crates.io/crates/fixedstr),
[`arrayvec::ArrayString`](https://crates.io/crates/arrayvec), and the semi-official
[`heapless::String`](https://docs.rs/heapless/latest/heapless/string/type.String.html). They lack a `heapless::Str`, to
match the faster fixed size `Stringlet`. That would be given by
[`fixedstr::zstr`](https://docs.rs/fixedstr/latest/fixedstr/struct.zstr.html) but their equality checks are not
optimized. I hope it can be independently confirmed (or debunked, if I mismeasured) that for tasks like `== Self` or `==
&str` all variants in this crate seem by a factor faster than competitors.

```rust
# extern crate stringlet;
use stringlet::{Stringlet, VarStringlet, TrimStringlet, SlimStringlet, stringlet};

let a: VarStringlet<10> = "shorter".into(); // override default Stringlet size of 16 and don’t use all of it
let b = a;
println!("{a} == {b}? {}", a == b);      // No “value borrowed here after move” error 😇

let nothing = Stringlet::new();          // Empty and zero size
let nil = VarStringlet::<5>::new();      // Empty and size 5 – impossible for fixed size Stringlet
let nada = TrimStringlet::<1>::new();    // Empty and size 1 – biggest an empty one can be

let x = stringlet!("Hello Rust!");       // Stringlet<11>
let y = stringlet!(v 14: "Hello Rust!"); // abbreviated VarStringlet<14>, more than length
let z = stringlet!(slim: "Hello Rust!"); // SlimStringlet<11>
let Ψ = stringlet!(v: ["abcd", "abc", "ab"]); // VarStringlet<4> for each
let ω = stringlet!(["abc", "def", "ghj"]); // Stringlet<3> for each

const HELLO: Stringlet<11> = stringlet!("Hello Rust!"); // Input length must match type
const PET: [Stringlet<3>; 4] = stringlet!(["cat", "dog", "ham", "pig"]); // size of 1st element
const PETS: [VarStringlet<8>; 4] = stringlet!(_: ["cat", "dog", "hamster", "piglet"]); // _: derive type
```

But

```text
error[E0277]: `VarStringlet<99>` or `SlimStringlet<99>` has excessive SIZE
  --> src/main.rs:99:16
   |
99 | let balloons = stringlet!(s 99: "Luftballons, auf ihrem…");
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ SIZE must be `0..=255` or `0..=64`
   |
   = help: the trait `Config<99, false>` is not implemented for `StringletBase<99, false>`
   = note: `VarStringlet` cannot be longer than 255 bytes. Consider using `String`!
   = note: `SlimStringlet` cannot be longer than 64 bytes. Consider using `VarStringlet`!
```

This is **not** your classical short string optimization (SSO) in so far as there is no overflow into an alternate
bigger storage. This is by design, as addressing two different data routes requires branching. On your hot path, branch
misprediction can be costly. Stringlet tries hard to be maximally branchless. The few `if`s and `||`s refer to
constants, so should be optimized away.

There is no `Option` or `Result` niche optimization yet. That should likewise be feasible for all stringlets with a
stored length. I only need to understand how to tell the compiler?

`Stringlet` is configured so it can only be instantiated with valid size. For normal use that’s all there is to it.
However when forwarding generic arguments to `Stringlet` you too have to specify `Config`. I wish I could just hide it
all behind `<const SIZE: usize<0..=64>>`!

Since we have configuration anyway, it can also apply alignments up to 64 to each instance where you may need this. This
is aliased to names like `Stringlet4` or `SlimStringlet8`. Do benchmark, e.g. on a modern x86 it no longer seems to
affect performance.

## Todo

- [ ] `StringletError` & `stringlet::Result`

- [ ] Run Miri on various architectures. Who’s willing to support with exotic stuff?

- [ ] Run `cargo mutants`

- [ ] Implement mutability, `+=`, `write!()`.

- [ ] Document!

- [ ] How to implement `Cow` / `Borrow` with `String` as owned type?

- [ ] Or rather a `Cow`-like storage-constrained/limitless pair that will transparently switch on overflow.

- [ ] Implement more traits.

- [x] Add a macro syntax for align.

- [ ] `format!()` equivalent `stringlet!(format …)` or `format_stringlet!()`

- [ ] Integrate into [string-rosetta-rs](https://github.com/rosetta-rs/string-rosetta-rs)

- [ ] Implement for popular 3rd party crates.

- [ ] Why does this not pick up the default SIZE of 16: `let fail = Stringlet::new();`

- [ ] Is there a downside to `Copy` by default?

- [ ] What’s our minimal rust-version?
