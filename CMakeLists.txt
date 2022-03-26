cmake_minimum_required(VERSION 3.0.0)
set(CMAKE_C_COMPILER /usr/bin/clang)
set(CMAKE_CXX_COMPILER /usr/bin/clang++)
project(solution VERSION 0.1.0)

include_directories(${CMAKE_SOURCE_DIR}/cxx/include)
aux_source_directory(${CMAKE_SOURCE_DIR}/cxx/src/ SRC)
add_executable(solution cxx/main.cc ${SRC})