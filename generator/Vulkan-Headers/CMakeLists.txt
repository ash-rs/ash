# ~~~
# Copyright (c) 2018 Valve Corporation
# Copyright (c) 2018 LunarG, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ~~~

cmake_minimum_required(VERSION 2.8.11)

project(Vulkan-Headers NONE)

include(GNUInstallDirs)
# Set a better default install location for Windows only if the user did not provide one.
if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT AND WIN32)
    set(CMAKE_INSTALL_PREFIX "${CMAKE_BINARY_DIR}/install" CACHE PATH "default install path" FORCE)
endif()

install(DIRECTORY "${CMAKE_SOURCE_DIR}/include/vulkan" DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
install(DIRECTORY "${CMAKE_SOURCE_DIR}/registry" DESTINATION ${CMAKE_INSTALL_DATADIR}/vulkan)

# uninstall target
if(NOT TARGET uninstall)
    configure_file("${CMAKE_CURRENT_SOURCE_DIR}/cmake/cmake_uninstall.cmake.in" "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
                   IMMEDIATE
                   @ONLY)
    add_custom_target(uninstall COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake)
endif()
