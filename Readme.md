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

## Guessing Game Takeaways

- User inputs
- Error handling while taking inputs. This is handled by `.expect(msg)`. Since, it checks with `Result` object. `Result` holds error and data when error did not occured.
- Comparison result taken care with `Ordering` which helps to handle all the possible cases of comparison.
- Threads for methods and functions to apply. Threads can be lazily initated which are seeded by the system.
- Cargo.lock for versioning and dependency management by cargo.

## Ownership

- A very important aspect of Rust. This makes sure the **safety** of the programs.
- Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time.
- The goal is to ensure that the program never have undefined behavior. This is because undefined behavior is dangerous to the low level programs with direct access to memory.
- Making the check in compile time to achieve fewer checks in runtime, thus improving the performance.
- Variables live in frames. Frames are organized into a stack of currently-called-functions.
- To transfer access to data without copying it, Rust uses pointers.
- The value that a pointer points-to is called its pointee.
- Rust provides a construct called `Box` for putting data on the heap.
- Rust do not allow to manually de-allocate the memory from heap.
- if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

## Referencing and Borrowing
- It does not pass the ownership to some other variable or function. Hence, after some process, it is not required to return the variable since it does not have ownership on it.
- The action of creating a reference is called `Borrowing`.
- References are by default immutable.
- Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
- A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.
