# STEP 0: Make sure there is no left-over profiling data from previous runs
rm -rf /tmp/pgo-data

# STEP 1: Build the instrumented binaries
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data -Ctarget-cpu=native" \
    cargo build --release --target=x86_64-unknown-linux-gnu

# STEP 2: Run the instrumented binaries with some typical data
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null
/tmp/cargo-nx-skl/x86_64-unknown-linux-gnu/release/aoc_2022 2&>/dev/null

# STEP 3: Merge the `.profraw` files into a `.profdata` file
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# STEP 4: Use the `.profdata` file for guiding optimizations
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Ctarget-cpu=native -Cllvm-args=-pgo-warn-missing-function" \
    cargo build --release --target=x86_64-unknown-linux-gnu
