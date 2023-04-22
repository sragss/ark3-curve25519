# ark3-curve25519

Implementation of an [arkworks-rs curve](https://github.com/arkworks-rs/curves) base / scalar fields forked from v0.3.0. Arkworks-rs added [curve25519 in v0.4.0](https://github.com/arkworks-rs/curves/tree/master/curve25519) but introduced a large number of [breaking changes](https://github.com/arkworks-rs/curves/blob/master/CHANGELOG.md) to the API.

[Zokrates](https://github.com/zokrates/zokrates) currently depends on Arkworks v0.3.0 across the board. Upgrading is non-trivial due to some deeper dependencies in Arkworks for unmaintained repositories ([ex](https://github.com/arkworks-rs/gm17)).

Ideally longer term Zokrates will upgrade holistically to v0.4.0 to include the cleaner serialization APIs and improved naming conventions, but for now this hacky solution is workable.
