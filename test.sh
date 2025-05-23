#!/bin/bash

PROBLEM="$1"
LANG="$2"

if ! [[ -d "problems/${PROBLEM}" ]]; then
  echo "Problem directory not found: problems/${PROBLEM}"
fi

if [[ "${LANG}" == "cpp" || "${LANG}" == "c++" ]]; then

  if ! [[ -d "problems/${PROBLEM}/cpp" ]]; then
    echo "Lang cpp does not exist for this problem. Hint: create a cpp directory inside the problem directory"
    exit 1

  fi

  cmake -B build -S . && make -C build && ./build/$PROBLEM
elif [[ "${LANG}" == "go" || "${LANG}" == "golang" ]]; then

  if ! [[ -d "problems/${PROBLEM}/go" ]]; then
    echo "Lang cpp does not exist for this problem. Hint: create a cpp directory inside the problem directory"
    exit 1

  fi

  go test "./problems/${PROBLEM}/go"
elif [[ "${LANG}" == "rust" ]]; then

  if ! [[ -d "problems/${PROBLEM}/rust" ]]; then
    echo "Lang cpp does not exist for this problem. Hint: create a cpp directory inside the problem directory"
    exit 1

  fi

  cargo test --bin "${PROBLEM}"
fi
