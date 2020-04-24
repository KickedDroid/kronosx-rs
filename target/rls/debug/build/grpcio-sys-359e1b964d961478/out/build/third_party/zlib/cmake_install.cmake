# Install script for directory: /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/zlib

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.1.2.11.dylib;/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.1.dylib")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib" TYPE SHARED_LIBRARY FILES
    "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/libz.1.2.11.dylib"
    "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/libz.1.dylib"
    )
  foreach(file
      "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.1.2.11.dylib"
      "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.1.dylib"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      execute_process(COMMAND "/usr/bin/install_name_tool"
        -id "libz.1.dylib"
        "${file}")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/usr/bin/strip" -x "${file}")
      endif()
    endif()
  endforeach()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.dylib")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib" TYPE SHARED_LIBRARY FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/libz.dylib")
  if(EXISTS "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.dylib" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.dylib")
    execute_process(COMMAND "/usr/bin/install_name_tool"
      -id "libz.1.dylib"
      "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.dylib")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -x "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.dylib")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib" TYPE STATIC_LIBRARY FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/libz.a")
  if(EXISTS "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/lib/libz.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/include/zconf.h;/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/include/zlib.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/include" TYPE FILE FILES
    "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/zconf.h"
    "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/zlib/zlib.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/share/man/man3/zlib.3")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/share/man/man3" TYPE FILE FILES "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/zlib/zlib.3")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/share/pkgconfig/zlib.pc")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
file(INSTALL DESTINATION "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/share/pkgconfig" TYPE FILE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/rls/debug/build/grpcio-sys-359e1b964d961478/out/build/third_party/zlib/zlib.pc")
endif()

