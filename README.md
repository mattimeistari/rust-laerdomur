# 🦀 RUST (me vs the compiler)

this repo is a documented beef between me and the rust compiler.  
sometimes i win. most times it reminds me who really in charge.

this is not a tutorial.  
this is a **survival log**.

---

## 🧠 what this repo really is

- me learning rust from the ground up
- me misunderstanding something, then understanding it 20 minutes later
- me fighting `borrow checker` like it owe me money
- me saying “ohhhh THAT’s why” way too often

every file in here represents growth, confusion, or both.

---

## 🧱 things rust already made me learn the hard way

- ownership is not a suggestion
- `mut` is a privilege, not a right
- `usize` exists solely to humble you
- strings are fake and `&str` is the truth
- the compiler not mean, it just don’t lie
- if it compiles, it probably right
- if it don’t compile, it *definitely* wrong

---

## 🛠️ tech i’ve touched (against my will)

- variables & mutability
- references (`&` / `&mut`)
- `Option` and why `None` is everywhere
- iterators (`map`, `filter`, `collect`)
- closures doing side quests
- input parsing with `std::io`
- removing duplicates, breaking strings, fixing mistakes
- out-of-bounds errors that *almost* got me

---

## 🧪 code in here be like

```rust
let result: String = input
    .trim()
    .chars()
    .filter(|&c| {
        let keep = Some(c) != prev;
        prev = Some(c);
        keep
    })
    .collect();
```

```rust
if love = true
return
else love = delete.system
```
