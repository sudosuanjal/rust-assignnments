# RUST ASSIGNMENTS

created by [`sudosuanjal`](https://x.com/sudosuanjal)  ||  [`github repo`](https://github.com/sudosuanjal/rust-assignnments)

## **Noob Level**

Covers: Variables, Mutability, Data Types, Strings, Functions, Control Flow

### **1. Temperature Converter**

Write a program that converts temperature from Celsius to Fahrenheit and vice versa.

- Use `let`, `mut`, and basic arithmetic.
- Take user input and call functions.

### **2. Even or Odd Detector**

Create a CLI app that takes an integer and checks whether it's even or odd.

- Use control flow (`if`, `else`) and `mod`.

### **3. Basic Calculator**

Build a command-line calculator that supports `+`, `-`, `*`, `/`.

- Use `match` for control flow, functions for each operation.

### **4. String Reverser**

Accept a string and print its reverse.

- Work with string slices and the `chars().rev()` iterator.

## **Beginner Level**

Covers: Structs, Enums, Ownership, Borrowing, References

### **5. Student Management System**

Define a `Student` struct with fields like name, age, and grade.

- Write functions to create, update, and display students.
- Practice borrowing and mutable references.

### **6. Simple Bank System**

Use an `enum` to define `Transaction::Deposit(f64)` and `Transaction::Withdraw(f64)`.

- Use ownership and pattern matching to update balance.

### **7. Library Book Tracker**

Create a program to track books using structs and enums.

- Handle `BookStatus` as an enum with variants like `Available`, `CheckedOut`.

## **Intermediate Level**

Covers: Move Semantics, Ownership Transfer, Traits, Option, Result

### **8. File Reader Tool**

Read a file and display line-by-line content.

- Return `Result<T, E>` from a function.
- Gracefully handle file-not-found errors.

### **9. To-do List with Ownership Rules**

Implement a to-do list manager with struct-based `Task`.

- Demonstrate move semantics by transferring tasks across vectors.
- Use `Option<Task>` for removing/completing tasks.

### **10. Trait-based Shape Area Calculator**

Define a `Shape` trait with an `area()` method. Implement it for `Circle`, `Rectangle`.

- Practice trait bounds and polymorphism.

## Advanced Level

Covers: Lifetimes, Complex Traits, Generic Types, Smart Pointers

### **11. Config Loader with Lifetime Annotations**

Write a struct `Config<'a>` that holds references to config values.

- Use lifetime annotations to ensure data safety.

### **12. Mini Command-Line Parser Library**

Create a generic function or struct to parse CLI args into a key-value format.

- Use `Option` and `Result` extensively.

### **13. Chat App Simulation (CLI)**

Simulate chat messages between users using structs and traits.

- Track message history with lifetimes & references.
- Use enums to represent message types.

### **14. Trait-based Plugin System**

Build a plugin interface with dynamic dispatch using `Box<dyn Trait>`.

- Load different modules with shared behavior.

### **15. Error Handling Library**

Create custom errors using the `Error` trait and `Result<T, MyError>`.

- Write fallbacks, retries, and unwrap safely.

===========================================================================

*Thanks a ton for checking out this curated list of Rust assignments! I'm currently learning Rust myself, and I’ll be updating the GitHub repo with my solutions as I complete each one.*

*Feel free to try the challenges yourself, share your solutions, or follow along as I document my Rust journey.*

*🔗 GitHub Repo: [https://github.com/sudosuanjal/rust-assignnments](https://github.com/sudosuanjal/rust-assignnments)
🐦 X (Twitter) Profile: [https://x.com/sudosuanjal](https://x.com/sudosuanjal)*

*Feel free to follow me to stay updated and connect. Let’s keep learning and building — one assignment at a time! 🦀🚀*

===========================================================================
