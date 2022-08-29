
cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME
    AoC-2021
)

set(SOURCE_DIR
    src
)

set(SOURCE
    ${SOURCE_DIR}/main.cpp
)

set(LIBRARIES_DIR
    libs
)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

enable_testing()
add_subdirectory(libs/gtest-1.12)
include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR})

add_executable(${PROJECT_NAME} ${SOURCE})