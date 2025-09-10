# DSA (Data Structures & Algorithm)

This repo contains all kinds of competitive programming algorithms/problems or algorithms in general that I learn and journal.

Language supported:
1. Golang
2. Rust
3. C++

## How to add a new solution 

Run below command to generate solution sub-directory for all supported laguages:

```shell
$ ./test.sh new reverseinteger
$ ls problems/reverseinteger --tree
problems/reverseinteger
├── cpp
│   └── solution.cc
├── go
│   ├── solution.go
│   └── solution_test.go
├── README.md
└── rust
    └── solution.rs
```

### To run C++ solution

Run below for all test cases:

```shell
./test.sh test reverseinteger c++
```

To run for a single test case:
```shell
./test.sh test reverseinteger c++ "Test Case#8"
```

### To run Golang solution

Run below for all test cases:

```shell
./test.sh test reverseinteger go 
```

To run for a single test case:
```shell
./test.sh test reverseinteger go -run TestProblem/Test_#3
```

### To run Rust solution

Run below for all test cases:

```shell
./test.sh test reverseinteger rust 
```

To run for a single test case:
```shell
./test.sh test reverseinteger rust tests::test_1
```


## Build processes of languages

### C++

cmake is used to build the test binary using [Catch2](https://github.com/catchorg/Catch2). For each new problem a new section like below is added to the CMakeLists.txt file.

```
add_executable(twosum problems/2sum/cpp/solution.cc)
target_link_libraries(twosum commons)
target_link_libraries(twosum Catch2::Catch2WithMain)
catch_discover_tests(twosum)
```

TODO:
- [ ] Run cmake build only for one problem. Currently, all binaries are built which would not be good as this project adds more solutions.

### Golang

The root of this repository is a go module. All problems are a package in itself which adds its own tests.

### Rust

The root of this repository is a cargo project. Each solution is added as a bin target in cargo.toml:

```toml
[[bin]]
name = "twosum"
path = "problems/2sum/rust/solution.rs"
```
