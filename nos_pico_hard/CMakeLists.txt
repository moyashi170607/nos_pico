# Generated Cmake Pico project file

cmake_minimum_required(VERSION 3.13)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Initialise pico_sdk from installed location
# (note this can come from environment, CMake cache etc)

# == DO NOT EDIT THE FOLLOWING LINES for the Raspberry Pi Pico VS Code Extension to work ==
if(WIN32)
    set(USERHOME $ENV{USERPROFILE})
else()
    set(USERHOME $ENV{HOME})
endif()
set(sdkVersion 2.2.0)
set(toolchainVersion 14_2_Rel1)
set(picotoolVersion 2.2.0-a4)
set(picoVscode ${USERHOME}/.pico-sdk/cmake/pico-vscode.cmake)
if (EXISTS ${picoVscode})
    include(${picoVscode})
endif()
# ====================================================================================
set(PICO_BOARD pico CACHE STRING "Board type")

# Pull in Raspberry Pi Pico SDK (must be before project)
include(pico_sdk_import.cmake)

project(nos_pico_hard C CXX ASM)

# Initialise the Raspberry Pi Pico SDK
pico_sdk_init()

# Add executable. Default name is the project name, version 0.1

add_executable(
    nos_pico_hard
    nos_pico_hard.cpp
    nostr_sign.cpp
    )

pico_set_program_name(nos_pico_hard "nos_pico_hard")
pico_set_program_version(nos_pico_hard "0.1")

# Modify the below lines to enable/disable output over UART/USB
pico_enable_stdio_uart(nos_pico_hard 0)
pico_enable_stdio_usb(nos_pico_hard 1)

# # --- secp256k1 用の設定 (ここから) ---

# --- secp256k1 のビルドオプションを強制的にオフにする ---
set(SECP256K1_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(SECP256K1_BUILD_BENCHMARK OFF CACHE BOOL "" FORCE)
set(SECP256K1_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(SECP256K1_BUILD_EXHAUSTIVE_TESTS OFF CACHE BOOL "" FORCE)

# すでに前の回答で設定済みかもしれませんが、念のためこれらも確認
set(SECP256K1_ENABLE_MODULE_RECOVERY ON CACHE BOOL "" FORCE)
set(SECP256K1_RANDOM_IT_SAFE OFF CACHE BOOL "" FORCE)

# # --- secp256k1 用の設定 (ここまで) ---

add_subdirectory(libs/secp256k1)

# Add the standard library to the build
target_link_libraries(nos_pico_hard
        pico_stdlib
        pico_rand
        secp256k1
        )

# Add the standard include files to the build
target_include_directories(nos_pico_hard PRIVATE
        ${CMAKE_CURRENT_LIST_DIR}
)

pico_add_extra_outputs(nos_pico_hard)

