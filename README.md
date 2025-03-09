
---

# **wrdlist**

A simple and efficient tool for generating custom word lists. Ideal for creating test data, pattern-based combinations, and structured text sequences.

---

## **Features**

- **Quickly generate custom word lists** with simple commands.
- **Easy-to-understand pattern syntax** for flexible generation.
- **View results** in the terminal or save them to a file.
- **Handle large outputs efficiently** with file size warnings.
- **Randomize or reverse the order** of results.

---

## **Installation**

### **Install Required Tools**

Before installing `wrdlist`, you need to install the necessary tools depending on your distribution.

#### **Arch Linux:**

```bash
sudo pacman -S rust
```

#### **Debian/Ubuntu:**

```bash
sudo apt install rustc cargo
```

#### **Fedora:**

```bash
sudo dnf install rust
```

---

### **Install `wrdlist` Using Cargo**

After installing the required tools, you can install `wrdlist` using the `cargo` tool (Rust's package manager):

```bash
cargo install wrdlist
```

To make sure `wrdlist` is globally accessible, you can copy it to `/usr/bin` and make it executable:

```bash
sudo cp ~/.cargo/bin/wrdlist /usr/bin
sudo chmod +x /usr/bin/wrdlist
```

---

## **Usage**

```bash
wrdlist [OPTIONS] <PATTERN> [OPTIONAL]
```

### **Arguments:**

- `<PATTERN>`  
    The pattern to generate the wordlist (must be enclosed in double quotes).

### **Options:**

- `-r, --random`  
    Randomize the order of the wordlist.

- `-i, --inverse`  
    Reverse the order of the wordlist.

- `-h, --help`  
    Print help information.

- `-v, --version`  
    Print version information.

### **Pattern Syntax:**

- **`.`** with a character to keep it fixed in every word (`.a` means 'a' stays in place).
- **`[start-end]`** for numeric ranges (`[1-3]` will generate 1, 2, 3).
- **`!`** for any lowercase letter (a-z).
- **`@`** for any uppercase letter (A-Z).
- **`#`** for any digit (0-9).
- **`%`** for any symbol (!@#$%^&*?).

### **Optional:**

- `-o, --output <OUTPUT>`  
    Save the wordlist to a file.

---

## **Example Usage**

```bash
# Generate a list of lowercase letters followed by digits
wrdlist "!#"

# Generate a list of numbers from 1 to 3, reversed
wrdlist -i "[1-5]"

# Randomize the order of a word list
wrdlist -r "[1-13].X.d"

...more in wrdlist --help 
```

> **Note:** Patterns must be enclosed in double quotes. For complex patterns, check how the output will look before continuing.

---

## **Performance**

This tool is designed to be lightweight and efficient. It calculates the expected file size before generating large word lists to avoid storage issues. You'll receive a warning if the output is too large, allowing you to decide whether to continue or cancel.

---
