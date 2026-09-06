### Classic Object-Oriented (OO) State Pattern

The Classic Object-Oriented (OO) State Pattern in Rust mimics the traditional
Gang of Four design. It uses a trait to define state behaviors and distinct
structs to represent each state.

Because Rust enforces strict memory safety and ownership rules, implementing
this pattern requires explicit handling of value ownership, usually via Box<dyn
State> and Option::take.

### Crucial Architectural Nuances in Rust

## 1. Why self: Box<Self> is Required

In traditional OO languages (like Java or C++), state transitions simply mutate
a pointer. In Rust, states often need to fully consume the previous state to
ensure it can no longer be used.

By using self: Box<Self>, the method takes ownership of the heap-allocated
state object, automatically dropping it out of memory when the transition
completes.

## 2. The Option::take() Trick

Rust requires that all fields in a struct are fully initialized at all times.
During a state transition, you cannot easily move a value out of self.state and
replace it in one line without violating this rule.

To bypass this, we wrap the state in an Option. Calling .take() pulls the state
value out, temporarily leaving None in its place, and allows us to pass
ownership safely into the transition methods.

## 3. Lifetime Interdependencies

Look at the content method signature:

```rust
```
fn content<'a>(&self, post: &'a Post) -> &'a str
```
```

The returned string slice (&str) references the content field owned by the Post
struct. The lifetime annotations ('a) guarantee to the compiler that the
returned text slice will not outlive the Post itself, preventing dangling
references.
