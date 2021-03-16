OUTPUT = /usr/local/musl

GCC_VER = 7.2.0

# MUSL_VER = 1.1.19
# GMP_VER =
# MPC_VER =
# MPFR_VER =
# ISL_VER =
# LINUX_VER =

# By default source archives are downloaded with wget. curl is also an option.

# DL_CMD = wget -c -O
DL_CMD = curl -C - -L -o

# Something like the following can be used to produce a static-linked
# toolchain that's deployable to any system with matching arch, using
# an existing musl-targeted cross compiler. This only # works if the
# system you build on can natively (or via binfmt_misc and # qemu) run
# binaries produced by the existing toolchain (in this example, i486).

# COMMON_CONFIG += CC="i486-linux-musl-gcc -static --static" CXX="i486-linux-musl-g++ -static --static"

# Recommended options for smaller build for deploying binaries:

COMMON_CONFIG += CFLAGS="-g0 -Os" CXXFLAGS="-g0 -Os" LDFLAGS="-s"

# Recommended options for faster/simpler build:

COMMON_CONFIG += --disable-nls
GCC_CONFIG += --enable-languages=c,c++
GCC_CONFIG += --disable-libquadmath --disable-decimal-float
GCC_CONFIG += --disable-multilib

# You can keep the local build path out of your toolchain binaries and
# target libraries with the following, but then gdb needs to be told
# where to look for source files.

COMMON_CONFIG += --with-debug-prefix-map=$(CURDIR)=
