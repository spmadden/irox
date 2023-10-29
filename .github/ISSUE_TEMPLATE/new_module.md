---
name: New Module Checklist
about: Checklist for a new module
title: 'New Module:'
labels: enhancement
assignees: ''

---

 - [ ] Update top level `irox` module:
    - [ ] Update `Cargo.toml`
    - [ ] Update `lib.rs`
 - [ ] Update top level `README.md`
 - [ ] Update top level `Cargo.toml`/`Cargo.lock`
   - [ ] Ensure new module in main module list
   - [ ] Ensure new module dependency hotlink exists
- [ ] In the new module:
  - [ ] Used the templates in `/dev/mod_template` 
  - [ ] Ensure `README.md` exists
  - [ ] Ensure `CHANGELOG.md` exists
  - [ ] Ensure `Cargo.toml` set up correctly
    - Ref: [Crates.io Category Slugs](https://crates.io/category_slugs)
  - [ ] Ensure `lib.rs`/`main.rs`:
    - [ ] Marked with `#![forbid(unsafe_code)]`
    - [ ] Has main module docs
- [ ] Module released
  - [ ] verify `just ci`
  - [ ] `cargo smart-release -u`
