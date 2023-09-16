cmake_minimum_required(VERSION 3.25)

project(libpci-rs-c-backend C CXX)

if(APPLE)
    add_library(libpci-rs-c-backend STATIC darwin/mod.cpp)
elseif(BSD STREQUAL "OpenBSD")
    add_library(libpci-rs-c-backend STATIC bsd/mod.c)
else()
    message(FATAL_ERROR "Attempt to compile on unknown target!")
endif()

target_include_directories(libpci-rs-c-backend PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS libpci-rs-c-backend DESTINATION .)
