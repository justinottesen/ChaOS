cmake_minimum_required(VERSION 4.1)

project(
    cha
    DESCRIPTION "A language"
    LANGUAGES CXX
    VERSION 0.1.0
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(cha)
target_sources(cha PRIVATE
    src/main.cpp
    src/error.cpp
)