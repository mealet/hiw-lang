# Dependencies
MINGW_INSTALL_COMMAND = apt install mingw-w64

# Build commands
LINUX_BUILD_COMMAND = cargo build --release
WINDOWS_BUILD_COMMAND = cargo build --target x86_64-pc-windows-gnu --release

# Build files and dirs
SOURCE_DIR = src

LINUX_EXE_FILE = hiw
WINDOWS_EXE_FILE = hiw.exe

LINUX_RELEASE_DIR = target/release
WINDOWS_RELEASE_DIR = target/x86_64-pc-windows-gnu/release

# Modules
MODULES_DIR = modules
VM_FILE = $(SOURCE_DIR)/vm.rs

# Output 
OUTPUT_DIR = hiw
OUTPUT_ARCHIVE = hiw-release.zip

ZIP_COMMAND = zip $(OUTPUT_ARCHIVE) $(OUTPUT_DIR)/*

release:
	sudo $(MINGW_INSTALL_COMMAND)
	$(LINUX_BUILD_COMMAND)
	$(WINDOWS_BUILD_COMMAND)
	mkdir $(OUTPUT_DIR)
	cp $(LINUX_RELEASE_DIR)/$(LINUX_EXE_FILE) $(OUTPUT_DIR)/$(LINUX_EXE_FILE)
	cp $(WINDOWS_RELEASE_DIR)/$(WINDOWS_EXE_FILE) $(OUTPUT_DIR)/$(WINDOWS_EXE_FILE)
	echo "Adding modules and dependencies"
	cp $(MODULES_DIR)/* $(OUTPUT_DIR)
	cp $(VM_FILE) $(OUTPUT_DIR)
	echo "Packing release..."
	$(ZIP_COMMAND)
	rm $(OUTPUT_DIR) -d -r
