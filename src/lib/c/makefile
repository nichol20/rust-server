CC      := gcc
AR      := ar
CFLAGS  := -O2 -Wall -Wextra -fPIC
SRCS    := mathlib.c
OBJS    := $(SRCS:.c=.o)
LIBNAME := libmathlib

.PHONY: all static shared test clean

all: static shared

# static library
static: $(OBJS)
	$(AR) rcs $(LIBNAME).a $(OBJS)

# shared library
shared: $(OBJS)
	$(CC) -shared -o $(LIBNAME).so $(OBJS)

%.o: %.c mathlib.h
	$(CC) $(CFLAGS) -c $< -o $@

# build and run tests against the static lib
test: static test_mathlib.o
	$(CC) test_mathlib.o -L. -lmathlib -o test_mathlib
	LD_LIBRARY_PATH=. ./test_mathlib

test_mathlib.o: test_mathlib.c mathlib.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.o *.a *.so test_mathlib
