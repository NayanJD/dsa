# DSA (Data Structures & Algorithm)

This repo contains all kinds of competitive programming algorithms/problems or algorithms in general that I learn and journal.

Language supported:
1. Golang
2. Rust
3. C++

## Golang

The root of this repository is a go module. All problems are a package in itself which adds its own tests.

### How to add a new solution

Just create a new directory in the problem directory called `go`. Add your solution in solution.go and your tests in solution_test.go.

## How to run

```shell
make test PROBLEM=twosum LANG=go
```

## Rust

The root of this repository is a cargo project. Each solution is added as a bin target in cargo.toml.

### How to add a new solution

Create a directory rust inside the new problem dirctory. Add solution.rs and add your tests right in solution.rs. Add the target in Cargo.toml:

```toml
[[bin]]
name = "twosum"
path = "problems/2sum/rust/solution.rs"
```

### How to run

```shell
make test PROBLEM=twosum LANG=rust
```

## C++

This uses Catch2 to validate different test cases.

### How to add a new solution

For a new C++ solution, add a folder cpp in the problem directory and add solution.cc into it. This .cc file should contain the solution along wit the test cases. For the new problem, add its target in the `CMakeLists.txt` file at the root of this project. Taking example of problem `twosum`:

```
add_executable(twosum problems/2sum/cpp/solution.cc)
target_link_libraries(twosum commons)
target_link_libraries(twosum Catch2::Catch2WithMain)
catch_discover_tests(twosum)
```

The target name should be the name of the problem directory and the path should reflect that in add_executable.

### How to build and run

Run this at the root after you have added the target as described in the previous section:

```shell
make test PROBLEM=twosum LANG=cpp # or c++
```
