# Generated from trgen 0.23.27
rm -rf Cargo.lock
rm -rf target
find src/gen -mindepth 1 ! -name 'mod.rs' -exec rm -rf '{}' ';'
