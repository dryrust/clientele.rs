"""Check every declared Clientele feature and focused interactions with Cargo.

Run from any directory with Python 3.9+; optionally select --toolchain 1.97.0.
Only the standard library is needed. Fail immediately on a Cargo or graph error.
"""

import argparse
import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]
COMBINATIONS = (
    "std",
    "tracing",
    "std,tracing",
    "clap",
    "clap,tracing",
    "clap,color,tracing",
    "clap,tracing,tracing-subscriber/ansi",
    "clap,dotenv",
    "clap,color,dotenv",
    "subcommands",
    "std,subcommands",
    "error-stack",
    "std,error-stack",
    "gofer",
    "completions",
    "manpages",
    "completions,manpages",
    "std,dirs,camino",
    "std,getenv,camino",
    "serde,camino",
    "serde-json",
    "std,serde-json",
    "std,argfile",
    "std,wild",
    "std,argfile,wild",
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", help="Rustup toolchain used for every Cargo invocation")
    options = parser.parse_args()
    cargo = ["cargo"]
    if options.toolchain:
        cargo.append("+" + options.toolchain)

    def run(*arguments):
        subprocess.run(cargo + list(arguments), cwd=ROOT, check=True)

    def capture(*arguments):
        return subprocess.check_output(cargo + list(arguments), cwd=ROOT, encoding="utf-8")

    metadata = json.loads(capture("metadata", "--no-deps", "--format-version", "1", "--locked"))
    package = next(package for package in metadata["packages"] if package["name"] == "clientele")
    base = ["-p", "clientele", "--no-default-features", "--locked", "--quiet"]
    for feature in [None, *sorted(package["features"])]:
        print(f"Checking standalone feature: {feature or '(none)'}", flush=True)
        selection = ["--features", feature] if feature else []
        # Keep the library-only invocation separate: dev dependencies can unify
        # features in the subsequent all-target check and mask missing edges.
        run("check", "--lib", *base, *selection)
        run("check", "--all-targets", *base, *selection)

    for features in [None, *COMBINATIONS]:
        print(f"Testing feature combination: {features or '(none)'}", flush=True)
        selection = ["--features", features] if features else []
        run("test", *base, *selection)

    for feature, forbidden in {
        "serde": {"serde_json", "camino"},
        "color": {"clap", "tracing-subscriber"},
        "unicode": {"clap"},
        "std": {"clap", "getenv", "gofer", "error-stack", "tracing-core", "tracing-subscriber"},
    }.items():
        tree = capture(
            "tree", "-p", "clientele", "--no-default-features", "--features", feature,
            "--locked", "--edges", "normal", "--prefix", "none", "--format", "{p}",
        )
        dependencies = {line.split()[0] for line in tree.splitlines() if line.strip()}
        unexpected = dependencies & forbidden
        if unexpected:
            raise SystemExit(f"{feature} unexpectedly enables: {', '.join(sorted(unexpected))}")
    print("All standalone features, focused combinations, and weak-feature guards passed.")


if __name__ == "__main__":
    main()
