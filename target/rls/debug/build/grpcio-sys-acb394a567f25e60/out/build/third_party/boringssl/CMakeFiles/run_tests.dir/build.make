# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.16

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Remove some rules from gmake that .SUFFIXES does not remove.
SUFFIXES =

.SUFFIXES: .hpux_make_needs_suffix_list


# Suppress display of executed commands.
$(VERBOSE).SILENT:


# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/Cellar/cmake/3.16.2/bin/cmake

# The command to remove a file.
RM = /usr/local/Cellar/cmake/3.16.2/bin/cmake -E remove -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build

# Utility rule file for run_tests.

# Include the progress variables for this target.
include third_party/boringssl/CMakeFiles/run_tests.dir/progress.make

third_party/boringssl/CMakeFiles/run_tests: third_party/boringssl/ssl/test/bssl_shim
	cd /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc && /usr/local/bin/go run util/all_tests.go -build-dir /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build
	cd /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc && cd ssl/test/runner && /usr/local/bin/go test -shim-path /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/boringssl/ssl/test/bssl_shim

run_tests: third_party/boringssl/CMakeFiles/run_tests
run_tests: third_party/boringssl/CMakeFiles/run_tests.dir/build.make

.PHONY : run_tests

# Rule to build all files generated by this target.
third_party/boringssl/CMakeFiles/run_tests.dir/build: run_tests

.PHONY : third_party/boringssl/CMakeFiles/run_tests.dir/build

third_party/boringssl/CMakeFiles/run_tests.dir/clean:
	cd /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/boringssl && $(CMAKE_COMMAND) -P CMakeFiles/run_tests.dir/cmake_clean.cmake
.PHONY : third_party/boringssl/CMakeFiles/run_tests.dir/clean

third_party/boringssl/CMakeFiles/run_tests.dir/depend:
	cd /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/boringssl /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/boringssl /Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/boringssl/CMakeFiles/run_tests.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : third_party/boringssl/CMakeFiles/run_tests.dir/depend
