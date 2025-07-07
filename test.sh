#!/bin/bash

test() {
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

    go test -v "./problems/${PROBLEM}/go"
  elif [[ "${LANG}" == "rust" ]]; then

    if ! [[ -d "problems/${PROBLEM}/rust" ]]; then
      echo "Lang cpp does not exist for this problem. Hint: create a cpp directory inside the problem directory"
      exit 1

    fi

    cargo test --bin "${PROBLEM}"
  fi
}

new() {
  PROBLEM="$1"

  if [[ -d "problems/${PROBLEM}" ]]; then
    echo "A problem directory with name problems/${PROBLEM} already exists. Exiting."
    exit 1
  fi

  cp -r problems/_template problems/${PROBLEM}

  cat <<EOF >> CMakeLists.txt

add_executable(${PROBLEM} problems/${PROBLEM}/cpp/solution.cc)
target_link_libraries(${PROBLEM} commons)
target_link_libraries(${PROBLEM} Catch2::Catch2WithMain)
catch_discover_tests(${PROBLEM})
EOF
  
  cat <<EOF >> Cargo.toml

[[bin]]
name = "${PROBLEM}"
path = "problems/${PROBLEM}/rust/solution.rs"
EOF

}

COMMAND="$1"

if [[ "$COMMAND" == "test" ]]; then
  test $2 $3
elif [[ "$COMMAND" == "new" ]]; then
  new $2
fi
