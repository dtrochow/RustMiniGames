cmake_minimum_required(VERSION 3.0)

project(RustMiniGames)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/tools/cmake_rust")
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/bin)

enable_language(Rust)
include(CMakeCargo)

add_subdirectory("src/cli")
add_subdirectory("src/hello_world")
add_subdirectory("src/games")

