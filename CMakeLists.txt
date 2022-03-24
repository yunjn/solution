cmake_minimum_required(VERSION 3.0.0)
project(PasswordKeeper VERSION 0.1.0)

include_directories(${CMAKE_SOURCE_DIR}/cpp/include)
aux_source_directory(${CMAKE_SOURCE_DIR}/cpp/src/ SRC)
add_executable(solution cpp/main.cc ${SRC})