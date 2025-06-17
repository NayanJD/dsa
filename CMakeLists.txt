cmake_minimum_required(VERSION 3.16)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(2sum)

find_package(Catch2 3 REQUIRED)

add_library(commons SHARED languages/cpp/lib.cpp)

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)

add_executable(twosum problems/twosum/cpp/solution.cc)
target_link_libraries(twosum commons)
target_link_libraries(twosum Catch2::Catch2WithMain)
catch_discover_tests(twosum)

add_executable(threesum problems/threesum/cpp/solution.cc)
target_link_libraries(threesum commons)
target_link_libraries(threesum Catch2::Catch2WithMain)
catch_discover_tests(threesum)

add_executable(dungeongame problems/dungeongame/cpp/solution.cc)
target_link_libraries(dungeongame commons)
target_link_libraries(dungeongame Catch2::Catch2WithMain)
catch_discover_tests(dungeongame)

add_executable(substringsansrepeating problems/substringsansrepeating/cpp/solution.cc)
target_link_libraries(substringsansrepeating commons)
target_link_libraries(substringsansrepeating Catch2::Catch2WithMain)
catch_discover_tests(substringsansrepeating)
