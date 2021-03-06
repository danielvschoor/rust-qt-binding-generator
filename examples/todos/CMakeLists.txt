project (my_rust_qt_quick_project)

cmake_minimum_required(VERSION 3.10 FATAL_ERROR)
cmake_policy(SET CMP0046 NEW)
cmake_policy(SET CMP0063 NEW)
cmake_policy(SET CMP0071 NEW)
LIST(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

string(TOUPPER "${CMAKE_BUILD_TYPE}" CMAKE_BUILD_TYPE_UPPER)
if(CMAKE_BUILD_TYPE_UPPER STREQUAL DEBUG)
    set(RUST_TARGET_DIR target/debug/)
    set(RUST_BUILD_FLAG)
else()
    set(RUST_TARGET_DIR target/release/)
    set(RUST_BUILD_FLAG --release)
endif()

### find dependencies ###

include(FeatureSummary)
find_package(Cargo REQUIRED)
find_package(Rust REQUIRED)
set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
find_package(Threads REQUIRED)

set(QT_MIN_VERSION "5.6.0")
find_package(Qt5 ${QT_MIN_VERSION} CONFIG
    REQUIRED COMPONENTS Core Quick
)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
find_package(RustQtBindingGenerator REQUIRED)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

### build commands ###

SET(RUST_DIR "${CMAKE_CURRENT_SOURCE_DIR}/rust")
if (MSVC)
  SET(RUST_LIB "${RUST_DIR}/${RUST_TARGET_DIR}/rust.lib")
else()
  SET(RUST_LIB "${RUST_DIR}/${RUST_TARGET_DIR}/librust.a")
endif()

# generate c++ and rust code from bindings.json
add_custom_command(
  OUTPUT "${RUST_DIR}/src/interface.rs"
         "${CMAKE_CURRENT_SOURCE_DIR}/src/Bindings.h"
         "${CMAKE_CURRENT_SOURCE_DIR}/src/Bindings.cpp"
  COMMAND "${RustQtBindingGenerator_EXECUTABLE}" #--overwrite-implementation
            "${CMAKE_CURRENT_SOURCE_DIR}/bindings.json"
  DEPENDS bindings.json
)

# compile the rust code into a static library
add_custom_command(
  OUTPUT "${RUST_LIB}"
  COMMAND ${Cargo_EXECUTABLE} build ${RUST_BUILD_FLAG}
  DEPENDS rust/src/lib.rs
          rust/src/implementation.rs
          rust/src/interface.rs
  WORKING_DIRECTORY "${RUST_DIR}"
)
add_custom_target(rust_target DEPENDS "${RUST_LIB}")

list(APPEND Libs "${RUST_LIB}")
if (MSVC)
  list(APPEND Libs WS2_32.LIB Userenv.lib)
endif()
list(APPEND Libs Qt5::Core Qt5::Quick Threads::Threads ${CMAKE_DL_LIBS})
set(SRCS src/main.cpp src/Bindings.cpp "qml.qrc")
add_executable(todos ${SRCS})
add_dependencies(todos rust_target)
target_link_libraries(todos ${Libs})
set_target_properties(todos PROPERTIES
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
)
