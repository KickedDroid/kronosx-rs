# Install script for directory: /Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/cares/cares

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out")
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
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/lib/libcares.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libcares.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libcares.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libcares.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/cares/cares/ares.h"
    "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/cares/cares/ares_version.h"
    "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/cares/cares/ares_dns.h"
    "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/ares_build.h"
    "/Users/aidanm/.cargo/registry/src/github.com-1ecc6299db9ec823/grpcio-sys-0.4.7/grpc/third_party/cares/cares/ares_rules.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares/c-ares-targets.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares/c-ares-targets.cmake"
         "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/CMakeFiles/Export/lib/cmake/c-ares/c-ares-targets.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares/c-ares-targets-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares/c-ares-targets.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares" TYPE FILE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/CMakeFiles/Export/lib/cmake/c-ares/c-ares-targets.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares" TYPE FILE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/CMakeFiles/Export/lib/cmake/c-ares/c-ares-targets-debug.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/c-ares" TYPE FILE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/c-ares-config.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/bin/ahost")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/ahost" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/ahost")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -u -r "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/ahost")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/bin/adig")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/adig" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/adig")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -u -r "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/adig")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE EXECUTABLE FILES "/Users/aidanm/Documents/GitHub/kronosx-rs/target/debug/build/grpcio-sys-acb394a567f25e60/out/build/third_party/cares/cares/bin/acountry")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/acountry" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/acountry")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" -u -r "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/bin/acountry")
    endif()
  endif()
endif()

