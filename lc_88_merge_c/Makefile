CC=gcc
CFLAGS=-c -Wall
BUILD_DIR=./build

all:
	mkdir -p $(BUILD_DIR)
	$(CC) -Wall tests.c tests.h merge.h -lcheck -o $(BUILD_DIR)/merge_88_tests

tests:
	$(BUILD_DIR)/merge_88_tests

clean:
	rm -fr $(BUILD_DIR)