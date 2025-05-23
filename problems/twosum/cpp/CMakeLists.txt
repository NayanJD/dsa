cmake_minimum_required(VERSION 3.16)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(2sum)

find_package(Catch2 3 REQUIRED)

add_executable(tests solution.cc)

add_library(commons SHARED ../../../languages/cpp/lib.cpp)

target_link_libraries(tests commons)
target_link_libraries(tests Catch2::Catch2WithMain)

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)
catch_discover_tests(tests)
