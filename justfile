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

test-msrv:
    @just test-msrv-with "1.50.0" "std"
    @just test-msrv-with "1.61.0" "std,msrv_1_61_0"
    @just test-msrv-with "1.89.0" "std,msrv_1_89_0"

[script]
test-msrv-with version features:
    msrv="$(whereis "cargo-msrv")"
    msrv="${msrv#"cargo-msrv: "}"
    msrv="$([[ -z $msrv ]] && echo "cargo msrv" || echo $msrv)"
    $msrv verify \
        --features "{{ features }}" \
        --rust-version "{{ version }}" \
        --no-default-features

[working-directory: 'fuzz']
fuzz target:
    RUSTFLAGS="${RUSTFLAGS:-} -C target-cpu=native" cargo fuzz run {{ target }}
    
# Run a command on the target host in `remote_dir`
ssh-run target +cmd: (ssh-copy target)
    ssh -t {{ target }} 'cd "{{ remote_dir }}" ; nix develop --command {{ cmd }}'

# Copy workspace to target into `remote_dir`
ssh-copy target:
    rsync -avP -e ssh --delete --exclude 'target' --exclude '.git' --filter=':- .gitignore' . "{{ target }}:{{ remote_dir }}"
