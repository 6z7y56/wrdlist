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

```bash
# Install using Cargo (Recommended)
cargo install wrdlist

# Make the tool globally accessible
sudo cp ~/.cargo/bin/wrdlist /usr/bin
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
wrdlist -i "[1-3]"

# Randomize the order of a word list
wrdlist -r "[1-6]"
```

> **Note:** Patterns must be enclosed in double quotes. For complex patterns, check how the output will look before continuing.

---

## **Performance**

This tool is designed to be lightweight and efficient. It calculates the expected file size before generating large word lists to avoid storage issues. You'll receive a warning if the output is too large, allowing you to decide whether to continue or cancel.

---
