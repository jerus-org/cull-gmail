#!/bin/sh

# Build an updated README
cat ../../docs/readme/head.md > README.md
# shellcheck disable=SC2129
cat ../../docs/main.md >> README.md
cat ../../docs/lib.md >> README.md
cat ../../docs/readme/tail.md >> README.md
# Also update workspace root README for GitHub display
cp README.md ../../README.md

# Build Changelog
gen-changelog generate \
    --display-summaries \
    --name "CHANGELOG.md" \
    --package "cull-gmail" \
    --repository-dir "../.." \
    --next-version "${NEW_VERSION:-${SEMVER}}"
