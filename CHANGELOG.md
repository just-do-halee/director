<!-- next-header -->

## [unreleased] - ReleaseDate


## [0.5.0] - 2022-09-17

### BREAKING CHANGES

- Removed `lazy_static!` crate.

## Improved

- Fully `#![no_std]` without any of features option.

## Fixed

- `#[director::main(any_not_std_mod)]` is now properly available.

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.4.0] - 2022-09-16

### BREAKING CHANGES

- Supported `#![no_std]`.
- New attributes macro `#[director::main(`_MUTEX_PARENT_MOD_`)]`.
- It can now configure its `Mutex` and `MutexGuard` like a cartridge.

  Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.3.0] - 2022-09-14

### BREAKING CHANGES

- Renamed Self::lock_sup\_\_\[STATE]() to Self::lock_sup`er`\_\_\[STATE]().

Released by [@just-do-halee](https://github.com/just-do-halee).

## [0.1.0] - 2022-09-14

Released by [@just-do-halee](https://github.com/just-do-halee).

<!-- next-url -->

[unreleased]: https://github.com/just-do-halee/director/compare/v0.5.0...HEAD

[0.5.0]: https://github.com/just-do-halee/director/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/just-do-halee/director/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/just-do-halee/director/compare/v0.1.0...v0.3.0
[0.1.0]: https://github.com/just-do-halee/director/compare/v0.1.0...v0.1.0
