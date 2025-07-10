cmake_minimum_required(VERSION 3.14)
project(redsys-backend VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard (C++20 for enterprise performance as specified in README)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Build type
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Compiler flags for enterprise deployment (optimized for GPU compute marketplace)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -DNDEBUG -march=native")

# Find required packages (Drogon image has all dependencies pre-installed)
find_package(Drogon 1.9.11 REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(Threads REQUIRED)

# Find optional packages
find_package(OpenSSL)
find_package(ZLIB)

# Add include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../shared/include)

# Source files
set(SOURCES
    src/main.cpp
    src/database.cpp
    src/marketplace_service.cpp
    src/job_orchestrator.cpp
    src/oauth2_middleware.cpp
)

# Header files
set(HEADERS
    include/database.h
    include/marketplace_service.h
    include/job_orchestrator.h
    include/oauth2_middleware.h
)

# Create executable
add_executable(redsys-backend ${SOURCES} ${HEADERS})

# Set target properties
set_target_properties(redsys-backend PROPERTIES
    CXX_STANDARD 20
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
    POSITION_INDEPENDENT_CODE ON
)

# Link libraries
target_link_libraries(redsys-backend PRIVATE 
    Drogon::Drogon 
    PostgreSQL::PostgreSQL
    Threads::Threads
)

# Optional libraries
if(OpenSSL_FOUND)
    target_link_libraries(redsys-backend PRIVATE OpenSSL::SSL OpenSSL::Crypto)
endif()

if(ZLIB_FOUND)
    target_link_libraries(redsys-backend PRIVATE ZLIB::ZLIB)
endif()

# Compiler-specific flags
target_compile_features(redsys-backend PRIVATE cxx_std_20)

# Installation
install(TARGETS redsys-backend
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)

# Install configuration files
install(FILES ${CMAKE_CURRENT_SOURCE_DIR}/config.json
    DESTINATION etc/redsys
)

# Testing (if enabled)
option(BUILD_TESTS "Build tests" OFF)
if(BUILD_TESTS)
    enable_testing()
    find_package(GTest)
    if(GTest_FOUND)
        add_subdirectory(tests)
    endif()
endif()

# Documentation (if enabled)
option(BUILD_DOCS "Build documentation" OFF)
if(BUILD_DOCS)
    find_package(Doxygen)
    if(Doxygen_FOUND)
        set(DOXYGEN_IN ${CMAKE_CURRENT_SOURCE_DIR}/docs/Doxyfile.in)
        set(DOXYGEN_OUT ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile)
        
        configure_file(${DOXYGEN_IN} ${DOXYGEN_OUT} @ONLY)
        
        add_custom_target(docs ALL
            COMMAND ${DOXYGEN_EXECUTABLE} ${DOXYGEN_OUT}
            WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
            COMMENT "Generating API documentation with Doxygen"
            VERBATIM
        )
    endif()
endif()

# Print configuration summary
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Drogon version: ${Drogon_VERSION}")
message(STATUS "PostgreSQL: ${PostgreSQL_VERSION}")
message(STATUS "Build tests: ${BUILD_TESTS}")
message(STATUS "Build docs: ${BUILD_DOCS}") 