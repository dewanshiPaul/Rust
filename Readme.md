# Key Takeaways

## Introduction of Rust

- Naming convention of file: "{word1}_{word2}"
- `main()` is the entry point.
- To run a rust file. It needs to be compiled and then run it seperately.
- After compilation, it generates outputs
    -  a binary executable (.exe)
    -  a .pdb (in Windows only)
- (.exe) is an application whereas (.pdb) is used for debugging the application.
- Unlike languages like Python, Javascript. Rust is a "ahead_of_time" compiled time language. This means, even if someone does not have rust installed in the system. They can still run the executable file.

## Variables

- Variables are declared with keyword `let`.
- Variables are immutable due to speed and concurrency.
- To make it mutable, add keyword `mut` after `let`.
- The value set to mutable variable remains valid inside its scope.
- Constants are defined with keyword `const`.
- Constants naming convention is Uppercase and Underscore.
- Constants can be declared globally as compare to variables.
- Constants are set to only constant expression.
- Shadowing is the concept of redeclaring a variable of the same name of mutable variable, such that the second or the most recent value will be stored in the given scope.
- Define it with keyword `let` having same name as mutable one.
