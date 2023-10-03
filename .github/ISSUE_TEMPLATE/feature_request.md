---
name: Feature request
about: Add a new feature, change features, remove an old feature
title: Add a new feature, change features, remove an old feature
labels: enhancement
assignees: ''

---
## Version
0.42.x

## Add a new feature
```rust
struct CoolThing {}
```
I add `CoolThing` because it is super cool!

## Change features
```diff
- fn dist(v0: &[f32], v1: &[f32]) -> f32;
+ fn distance((v0, v1): (&[f32], &[f32])) -> f32;

- trait Vect {}
+ trait Vector {}
```
Reason: I prefer this

## Remove an old feature
```rust
fn obsolete_function();
```
Reason: not used
