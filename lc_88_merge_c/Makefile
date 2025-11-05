CC=gcc
CFLAGS=-Wall -Werror
BUILD_DIR=./build
BINARY_NAME=out

all:
	mkdir -p $(BUILD_DIR)
	$(CC) -Wall -g -fsanitize=address \
		tests.h \
		tests.c  \
		merge.h \
		merge.c \
		-lcheck \
		-o $(BUILD_DIR)/$(BINARY_NAME)

test:
	$(BUILD_DIR)/$(BINARY_NAME)

clean:
	rm -fr $(BUILD_DIR)