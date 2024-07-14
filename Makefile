release:
	sudo apt-get install mingw-w64
	cargo build --release
	cargo build --target x86_64-pc-windows-gnu --release
	mkdir hiw
	cp target/release/hiw hiw/hiw
	cp target/x86_64-pc-windows-gnu/release/hiw.exe hiw/hiw.exe
	echo "Adding modules and dependencies"
	cp modules/* hiw
	cp src/vm.rs hiw
	echo "Packing release..."
	zip hiw-release.zip hiw/*
	rm hiw -d -r