# Publish Checklist

## Publish to Cargo

This checklist is just here for me to reduce the friction of publishing new versions.

Code changes

1. Run `cargo update` to make sure dependencies are up to date
2. Run `./test.sh` to make sure that didn't break anything
3. Change the version in `Cargo.toml` and in this document (do a global find and replace)
4. Update `CHANGELOG.md` with the version number
5. Open PR for the version and wait for it to pass
6. Commit and merge PR
7. Publish to cargo

```bash
git checkout main
git pull
cargo publish
```
