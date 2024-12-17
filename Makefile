TARGET = release
BUILD_DIR = build
RELEASE_BINARY = target/$(TARGET)/LabelSoft.exe
OUTPUT_BINARY = LabelSoft.exe

all: build

build:
	cargo build --release
	mkdir -p $(BUILD_DIR)
clean:
	rm -r $(BUILD_DIR) target

.PHONY:	clean build
