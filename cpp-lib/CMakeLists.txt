cmake_minimum_required(VERSION 3.10)

project(ExampleCppLib VERSION 1.0)

# use C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# store output in target dir
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/target/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/target/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/target/bin)

# Cpp library
add_library(cpp-lib STATIC src/example2.cpp)

# C library
add_library(c-lib STATIC src/example.c)
