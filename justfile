remote_dir := "simd-adler32"

build *args:
    cargo build --release {{ args }}

[working-directory: 'bench']
bench *args:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-cpu=native" cargo bench {{ args }}

[working-directory: 'bench']
bench-wasm:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+simd128" cargo build --target wasm32-wasip1 --release --no-default-features --bench update
    wasmtime run --dir=. "$(ls -t target/wasm32-wasip1/release/deps/*.wasm | head -1)" --bench

test *args:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-cpu=native" cargo test {{ args }}

test-miri *args:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-cpu=native" cargo miri test {{ args }}

test-wasm:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-feature=+simd128" cargo build --target wasm32-wasip1 --tests
    wasmtime run "$(ls -t target/wasm32-wasip1/debug/deps/imp-*.wasm | head -1)" 
    
# Run a command on the target host in `remote_dir`
ssh-run target +cmd: (ssh-copy target)
    ssh -t {{ target }} 'cd "{{ remote_dir }}" ; nix develop --command {{ cmd }}'

# Copy workspace to target into `remote_dir`
ssh-copy target:
    rsync -avP -e ssh --delete --exclude 'target' --exclude '.git' --filter=':- .gitignore' . "{{ target }}:{{ remote_dir }}"
