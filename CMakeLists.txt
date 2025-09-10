cmake_minimum_required(VERSION 3.16)

# Reduce CMake verbosity
set(CMAKE_MESSAGE_LOG_LEVEL STATUS)
set_property(GLOBAL PROPERTY RULE_MESSAGES OFF)
set_property(GLOBAL PROPERTY TARGET_MESSAGES OFF)

# Force use of Clang 19
set(CMAKE_C_COMPILER clang-19)
set(CMAKE_CXX_COMPILER clang++-19)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -fsanitize=signed-integer-overflow -fno-omit-frame-pointer not used for now
set(CMAKE_CXX_FLAGS "-g -O2 -fsanitize=address -fsanitize=undefined")

project(2sum)

find_package(Catch2 3 REQUIRED)

add_library(commons SHARED languages/cpp/lib.cpp)

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)

# Require PROBLEM_NAME to be specified
if(NOT DEFINED PROBLEM_NAME)
    message(FATAL_ERROR "PROBLEM_NAME must be specified. Use: cmake -DPROBLEM_NAME=<problem_name>")
endif()

if(PROBLEM_NAME STREQUAL "twosum")
  add_executable(twosum problems/twosum/cpp/solution.cc)
  target_link_libraries(twosum commons)
  target_link_libraries(twosum Catch2::Catch2WithMain)
  catch_discover_tests(twosum)
endif()

if(PROBLEM_NAME STREQUAL "threesum")
  add_executable(threesum problems/threesum/cpp/solution.cc)
  target_link_libraries(threesum commons)
  target_link_libraries(threesum Catch2::Catch2WithMain)
  catch_discover_tests(threesum)
endif()

if(PROBLEM_NAME STREQUAL "dungeongame")
  add_executable(dungeongame problems/dungeongame/cpp/solution.cc)
  target_link_libraries(dungeongame commons)
  target_link_libraries(dungeongame Catch2::Catch2WithMain)
  catch_discover_tests(dungeongame)
endif()

if(PROBLEM_NAME STREQUAL "substringsansrepeating")
  add_executable(substringsansrepeating problems/substringsansrepeating/cpp/solution.cc)
  target_link_libraries(substringsansrepeating commons)
  target_link_libraries(substringsansrepeating Catch2::Catch2WithMain)
  catch_discover_tests(substringsansrepeating)
endif()

if(PROBLEM_NAME STREQUAL "containermostwater")
  add_executable(containermostwater problems/containermostwater/cpp/solution.cc)
  target_link_libraries(containermostwater commons)
  target_link_libraries(containermostwater Catch2::Catch2WithMain)
  catch_discover_tests(containermostwater)
endif()

if(PROBLEM_NAME STREQUAL "reverseinteger")
  add_executable(reverseinteger problems/reverseinteger/cpp/solution.cc)
  target_link_libraries(reverseinteger commons)
  target_link_libraries(reverseinteger Catch2::Catch2WithMain)
  catch_discover_tests(reverseinteger)
endif()

message(STATUS "Using Clang 19 compiler")
message(STATUS "Problem: ${PROBLEM_NAME}")
message(STATUS "Debug flags: ${CMAKE_CXX_FLAGS}")
