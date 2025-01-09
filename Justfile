mod? local

# An alias for cargo +nightly xtask check
powerset *args:
    cargo +nightly xtask powerset {{args}}

# An alias for cargo +nightly fmt --all
fmt *args:
    cargo +nightly fmt --all {{args}}

lint *args:
    cargo +nightly clippy --fix --allow-dirty --all-targets --allow-staged {{args}}

test *args:
    #!/bin/bash
    set -euo pipefail

    # We use the nightly toolchain for coverage since it supports branch & no-coverage flags.
    if [ -z ${RUST_TOOLCHAIN+x} ]; then
        export RUSTUP_TOOLCHAIN=nightly
    fi

    INSTA_FORCE_PASS=1 cargo llvm-cov clean --workspace
    INSTA_FORCE_PASS=1 cargo llvm-cov nextest --include-build-script --no-report -- {{args}}
    # Coverage for doctests is currently broken in llvm-cov.
    # Once it fully works we can add the `--doctests` flag to the test and report command again.
    cargo llvm-cov test --doc --no-report -- {{args}}

    # Do not generate the coverage report on CI
    cargo insta review
    cargo llvm-cov report --lcov --output-path ./lcov.info
    cargo llvm-cov report --html

deny *args:
    cargo deny {{args}} --all-features check

workspace-hack:
    cargo hakari manage-deps
    cargo hakari generate
