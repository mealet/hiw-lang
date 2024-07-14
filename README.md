<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/mealet/hiw/assets/110933288/dc24f25f-707e-472d-8305-f66a6d147670">
    <source media="(prefers-color-scheme: light)" srcset="https://github.com/mealet/hiw/assets/110933288/0cf160ac-ec86-4e24-8b15-05d732c712a6">
    <img src="https://github.com/mealet/hiw/assets/110933288/0cf160ac-ec86-4e24-8b15-05d732c712a6" width="50%"/>
  </picture>

  [Release] | [Quick Start] | [Installation] | [Documentation] | [Editors Support] | [License]

  A simple compiling language written in Rust
</div>

[Release]: https://github.com/mealet/hiw-lang/releases/latest
[Installation]: https://github.com/mealet/hiw-lang?tab=readme-ov-file#--installation
[Quick Start]: https://github.com/mealet/hiw-lang?tab=readme-ov-file#--quick-start
[Documentation]: https://github.com/mealet/hiw-lang/wiki/Documentation
[Editors Support]: https://github.com/mealet/hiw-lang?tab=readme-ov-file#--editors-support
[License]: https://github.com/mealet/hiw-lang/blob/master/LICENSE

## 👾 | Quick Start
Steps to start writing on hiw-lang:
1. **Install** language (see in [Installation])
2. Create file with `.hiw` extension
3. Write your first program:

```cpp
// example.hiw

// Hello World program
print("Hello World!");

// Creating variables
a = 1;
b = 2;
c = a + b;

str1 = "Hello, ";
str2 = "World!";
print(str1 + str2);

// If/Else constructions
a = 1;
if (a < 5) {
  print("less");
} else {
  print("bigger");
};

// Create functions
define foo(x) {
  print(x);
};

a = 2;

foo(1);
foo(a);

// Use functions on variables too!

a.foo();

// Use dynamic arrays
arr = [1, "string", true];

// Use cycle
a = 0;
while (a < 5) {
  print(a);
  a = a + 1;
};
```
4. Compile and run it:
```
hiw example.hiw
```
5. Compile it to binary file:
```
hiw example.hiw output
./output
```

## 💾 | Installation
#### 🟠 | Linux
1. Open your terminal.
2. Paste auto-install script:
```bash
curl -s https://raw.githubusercontent.com/mealet/hiw-lang/master/install.sh | sh
```
3. Restart the terminal and type `hiw` command. You'll see instructions about using compiler.
4. Write code!

#### 🟢 | Windows
1. Install Rust from [official site](https://www.rust-lang.org/)
2. Download **hiw** from [latest release][Release] and unpack it anywhere.
3. Add directory where you unpacked release to [PATH](https://stackoverflow.com/questions/44272416/how-to-add-a-folder-to-path-environment-variable-in-windows-10-with-screensho)
4. Restart the terminal and type `hiw` command. You'll see instructions about using compiler.
5. Write code!

## 😞 | Uninstall
#### 🟠
1. Open your terminal
2. Paste this command:
```bash
curl -s https://raw.githubusercontent.com/mealet/hiw-lang/master/uninstall.sh | sh
```
3. Restart terminal

#### 🟢 Windows
1. Delete unpacked _hiw_ folder with all binaries inside.

## 📒 | Editors Support
Editors which supports **hiw-lang** syntax:
* [Neovim](https://github.com/mealet/hiw/blob/master/syntax-highlight/neovim/Neovim%20Syntax%20Support.md)
* [VS Code](https://marketplace.visualstudio.com/items?itemName=mealet.hiw-language)

## 🎈 | License
Project licensed under the MIT License -> [License File][License]
