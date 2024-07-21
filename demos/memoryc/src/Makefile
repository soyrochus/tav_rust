# Makefile for the simple C program demonstrating a memory error

# Compiler to use
CC = gcc

# Compiler flags
CFLAGS = -Wall -Wextra -Werror -g

# Name of the executable
TARGET = memory_error_example

# Source files
SRCS = main.c

# Object files (generated from source files)
OBJS = $(SRCS:.c=.o)

# Default target to build the program
all: $(TARGET)

# Rule to link object files to create the executable
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

# Rule to compile source files into object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up generated files
clean:
	rm -f $(OBJS) $(TARGET)

# Phony targets to prevent conflicts with files named 'clean' or 'all'
.PHONY: all clean
