load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

RUST_VERSION = "1.60.0"
RUST_EDITION = "2018"

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "39655ab175e3c6b979f362f55f58085528f1647957b0e9b3a07f81d8a9c3ea0a",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.2.0/rules_rust-v0.2.0.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.2.0/rules_rust-v0.2.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(version=RUST_VERSION, edition=RUST_EDITION)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate")

crates_repository(
    name = "crate_index",
    lockfile = "//:Cargo.Bazel.lock",
    manifests = ["//:Cargo.toml"],
    # Should match the version represented by the currently registered `rust_toolchain`.
    rust_version = RUST_VERSION,
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()