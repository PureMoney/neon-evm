#!/bin/bash

cat << EOF
  - label: ":docker: trigger python logged groups pipeline"
    trigger: "rozhkovdmitrii-sanbox-pipeline"

    build:
      branch: "play-buildkite"
      env:
          EVM_LOADER_REVISION: "$${BUILDKITE_COMMIT}"
          EVM_LOADER_BRANCH: "$${EVM_LOADER_BRANCH}"
          SOLANA_REVISION: "v1.8.12-testnet"
          FULL_TEST_SUITE: $(buildkite-agent meta-data get "full_test_suite" --default "false")
EOF

