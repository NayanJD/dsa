#!/bin/bash

test() {
  PROBLEM="$1"
  LANG="$2"
  # Shift the first two arguments so $@ contains all remaining arguments
  shift 2

  # Now $@ contains all additional arguments

  if ! [[ -d "problems/${PROBLEM}" ]]; then
    echo "Problem directory not found: problems/${PROBLEM}"
  fi

  if [[ "${LANG}" == "cpp" || "${LANG}" == "c++" ]]; then

    if ! [[ -d "problems/${PROBLEM}/cpp" ]]; then
      echo "Lang cpp does not exist for this problem. Hint: create a cpp directory inside the problem directory"
      exit 1

    fi
    
    # Build with cmake -DCMAKE_VERBOSE_MAKEFILE=ON to show cmake commands
    cmake -B build -DCMAKE_VERBOSE_MAKEFILE=ON -S . && cmake --build build && ./build/$PROBLEM "$@"
  elif [[ "${LANG}" == "go" || "${LANG}" == "golang" ]]; then

    if ! [[ -d "problems/${PROBLEM}/go" ]]; then
      echo "Lang go does not exist for this problem. Hint: create a go directory inside the problem directory"
      exit 1

    fi

    go test -v "./problems/${PROBLEM}/go" "$@"
  elif [[ "${LANG}" == "rust" ]]; then

    if ! [[ -d "problems/${PROBLEM}/rust" ]]; then
      echo "Lang rust does not exist for this problem. Hint: create a rust directory inside the problem directory"
      exit 1

    fi

    cargo test --bin "${PROBLEM}" "$@"
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
  shift 1
  test "$@" 
elif [[ "$COMMAND" == "new" ]]; then
  shift 1
  new "$@"
fi
