# Building a Shell in Rust (Codecrafters)

This document contains my notes, observations, mistakes, and learnings while building a Unix shell from scratch in Rust using Codecrafters.

---

# Stage 1: Printing a Prompt

## Goal

Display a shell prompt (`$`) and wait for user input.

Expected behavior:

```text
$
```

---

## Rust Concepts Learned

### Importing I/O Utilities

```rust
use std::io::{self, Write};
```

* `std::io` provides input/output functionality.
* `Write` is a trait that provides methods such as:

  * `flush()`
  * `write()`
  * `write_all()`

---

### `print!` vs `println!`

```rust
print!("$ ");
```

Use `print!` instead of `println!`.

Why?

`println!` moves the cursor to the next line:

```text
$
cursor here
```

`print!` keeps the cursor on the same line:

```text
$ cursor here
```

This is how real shells behave.

---

### Understanding `stdout()`

```rust
io::stdout()
```

Think of it as:

> Give me access to the terminal output stream.

---

### Understanding `flush()`

```rust
io::stdout().flush()
```

Output is often buffered.

Without flushing, the prompt might not appear immediately.

```rust
print!("$ ");
io::stdout().flush().unwrap();
```

`flush()` forces buffered output to be displayed immediately.

---

### Why `unwrap()`?

`flush()` returns:

```rust
Result<(), std::io::Error>
```

Possible outcomes:

| Result       | Meaning              |
| ------------ | -------------------- |
| `Ok(())`     | Success              |
| `Err(error)` | Something went wrong |

Examples of failures:

* Output stream closed
* OS error
* Permission issue

`unwrap()` means:

> Continue if successful, crash if not.

---

## Takeaway

A shell prompt seems simple, but it introduced:

* Standard output
* Buffering
* Traits
* Result types
* Error handling

---

# Stage 2: Handling Invalid Commands

## Goal

Print an error message for any command entered by the user.

Example:

```text
$ xyz
xyz: command not found
```

---

## What I Learned

A shell must:

1. Display a prompt
2. Read user input
3. Decide what the command means
4. Print a result

At this stage every command is considered invalid.

---

# Stage 3: Implementing a REPL

## What is a REPL?

REPL stands for:

| Letter | Meaning  |
| ------ | -------- |
| R      | Read     |
| E      | Evaluate |
| P      | Print    |
| L      | Loop     |

A shell repeatedly performs these steps:

```text
Prompt
 ↓
Read Input
 ↓
Evaluate Command
 ↓
Print Result
 ↓
Repeat
```

---

## My Solution

I wrapped the existing shell logic in:

```rust
loop {
    // shell logic
}
```

This keeps the shell running until explicitly terminated.

---

## Takeaway

A shell is fundamentally just an infinite loop that repeatedly processes user input.

---

# Stage 4: Implementing `exit`

## Goal

Allow the shell to terminate when the user enters:

```text
exit
```

---

## Builtins vs Executables

One of the most important concepts I learned.

### Builtin Commands

Builtin commands are handled directly by the shell.

Examples:

```text
exit
cd
pwd
```

Characteristics:

| Builtin             | Description |
| ------------------- | ----------- |
| Runs inside shell   | Yes         |
| New process created | No          |

---

### Executable Commands

Executable commands are programs stored as files on disk.

Examples:

```text
ls
cat
grep
vim
```

When running:

```text
$ ls
```

the shell:

1. Finds the executable file
2. Starts a new process
3. Waits for it to finish
4. Displays the prompt again

---

## My Solution

After trimming user input:

```rust
if user_input == "exit" {
    break;
}
```

Breaking the loop terminates the shell.

---

# Stage 5: Implementing `echo`

## Goal

Print everything after the word `echo`.

Example:

```text
$ echo hello world
hello world
```

---

## Key Idea

Detect commands beginning with:

```rust
user_input.starts_with("echo ")
```

Then print everything after the first five characters.

---

## What I Learned

String slicing:

```rust
&user_input[5..]
```

creates a string slice without copying data.

---

# Stage 6: Implementing `type`

## Goal

Determine how a command would be interpreted by the shell.

Examples:

```text
$ type echo
echo is a shell builtin

$ type exit
exit is a shell builtin

$ type invalid_command
invalid_command: not found
```

---

## My Solution

```rust
["echo", "exit", "type"]
    .iter()
    .any(|&s| s == cmd)
```

---

## Understanding the Logic

### Array

```rust
["echo", "exit", "type"]
```

Contains known builtin commands.

---

### Iterator

```rust
.iter()
```

Visits each element one at a time.

---

### `any()`

```rust
.any(...)
```

Returns true if at least one element satisfies the condition.

Conceptually:

```rust
for item in array {
    if item == cmd {
        return true;
    }
}
```

---

## Takeaway

This stage introduced:

* Arrays
* Iterators
* Closures
* Membership checking

---

# Stage 7: Locating Executable Files

## Goal

Extend `type` so it can locate executable programs on the system.

Example:

```text
$ type grep
grep is /usr/bin/grep
```

---

## The PATH Environment Variable

PATH tells the shell where executable programs are stored.

Example:

```text
PATH=/usr/bin:/usr/local/bin
```

This means:

```text
Search /usr/bin first
 ↓
Search /usr/local/bin second
```

---

## How a Shell Uses PATH

When a command is not a builtin:

```text
User enters command
 ↓
Check first PATH directory
 ↓
Found?
 ↓
Yes → Stop
No → Check next directory
```

The search continues until:

* A valid executable is found
* All directories have been checked

---

## Important Observation

Order matters.

Example:

```text
PATH=/dir1:/dir2
```

If both directories contain a program with the same name:

```text
/dir1/python
/dir2/python
```

The shell will use:

```text
/dir1/python
```

because it appears first.

---

## Concepts Learned

* Environment variables
* Filesystem navigation
* Executable permissions
* PATH lookup
* Command discovery

---

# Progress Summary

| Stage | Topic            |
| ----- | ---------------- |
| 1     | Prompt           |
| 2     | Invalid Commands |
| 3     | REPL             |
| 4     | Exit Builtin     |
| 5     | Echo Builtin     |
| 6     | Type Builtin     |
| 7     | PATH Lookup      |

```
```
