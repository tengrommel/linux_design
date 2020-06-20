# Motivation for testing

Software systems are like machines with small cogs and gears. If any of the individual gears malfunctions, the machine as a whole is most likely to behave in an unreliable manner. 

In software, the individual gears are functions, modules, or any libraries that you use. Functional testing of the individual components of a software system is an effective and practical way of maintaining high quality code. It doesn't prove that bugs don't exist, but it helps in building confidence when deploying the code to production and maintaining the sanity of the code base when the project is to be maintained for a long time. Furthermore, large-scale refactoring in software is hard to do without unit tests. The benefits of the smart and balanced use of unit testing in software are profound. During the implementation phase, a well-written unit test becomes an informal specification for components of the software. In the maintenance phase, the existing unit tests serve as a harness against regressions in the code base, encouraging an immediate fix. 

In compiled languages like Rust, this gets even better as the refactors involved (if any) for regressions from unit tests are more guided due to helpful error diagnostics from the compiler.

Another good side effect of unit tests is that they encourage the programmer to write modular code that is mostly dependent on the input parameters, that is, stateless functions. It moves the programmer away from writing code that depends on a global mutable state. Writing tests that depend on a global mutable state are hard to write. Moreover, the act of simply thinking about writing tests for a piece of code helps the programmer figure out silly mistakes in their implementation. They also act as very good documentation for any newcomer trying to understand how different parts of the code base interact with each other.

Another good side effect of unit tests is that they encourage the programmer to write modular code that is mostly dependent on the input parameters, that is, stateless functions. It moves the programmer away from writing code that depends on a global mutable state. Writing tests that depend on a global mutable state are hard to write. Moreover, the act of simply thinking about writing tests for a piece of code helps the programmer figure out silly mistakes in their implementation. They also act as very good documentation for any newcomer trying to understand how different parts of the code base interact with each other.

The takeaway is that tests are indispensable for any software project. Now, let's look at how we can write tests in Rust, starting by learning about organizing tests!


# Using generics
>Now, the way we instantiate or use generic types is also a bit different than their non-generic counterparts. 

Any time we instantiate them, the compiler needs to know the concrete type in place of T in their type, signature, which gives it the type information to monomorphize the generic code. Most of the time, the concrete type is inferred based on the instantiation of the type or by calling any method that takes a concrete type in the case of generic functions. In rare cases, we need to help the compiler by specifically typing out the concrete type in place of the generic type by using the turbofish (::<>) operator. We'll see how that is used in a moment.

# Exploring standard library traits

Rust's standard library has a lot of built-in traits. Most of the syntatic sugar in Rust is due to traits. These traits also provide a nice baseline upon which crate authors can provide an idiomatic interface to their libraries. In this section, we'll explore some of the abstractions and conveniences of the standard library traits that enhance the experience for a crate author and the consumer. We'll base our exploration from a library author's perspective and create a library that provides support for complex number types. This example serves well to introduce the common traits you have to implement if you are creating a crate of your own.


We'll create a new project by running cargo new complex --lib. To start with, we need to represent our complex number as a type. We'll use a struct for this. Our complex number struct has two fields: the real and imaginary part of a complex number. Here's how we have defined it:

// complex/src/lib.rs

    struct Complex<T> {
        // Real part
        re: T,
        // Complex part
        im: T
    }

We're making it generic over T, as re and im can both be a float or an integer value. For this type to be of any use, we want ways to create instances of it. The usual way to do this is to implement the associated method new, where we pass the values for re and im. What if we also wanted to initialize a complex value with defaults (say re = 0, im = 0) ? For this, we have a trait called Default. Implementing Default is very simple for a user-defined type; we can just put a #[derive(Default)] attribute over the Complex structure to automatically implement the Default trait for it.

Now, our updated code with the method new and the Default annotation looks like this:

// complex/src/lib.rs

    #[derive(Default)]
    struct Complex<T> {
        // Real part
        re: T,
        // Complex part
        im: T
    }

    impl<T> Complex<T> {
        fn new(re: T, im: T) -> Self {
            Complex { re, im }
        }
    }

    #[cfg(test)]
    mod tests {
        use Complex;
        #[test]
        fn complex_basics() {
            let first = Complex::new(3,5);
            let second: Complex<i32> = Complex::default();
            assert_eq!(first.re, 3);
            assert_eq!(first.im, 5);
            assert!(second.re == second.im);
        }
    }

We also added a simple initialization test case at the bottom under the tests module. The #[derive(Default)] attribute functionality is implemented as a procedural macro that can automatically implement traits for the type on which it appear. This auto-deriving requires that the fields of any custom type, such as a struct or an enum, also implement the Default trait themselves. Deriving a trait using them is only applicable to structs, enums, and unions. We'll look at how to write our own deriving procedural macros in Chapter 9, Metaprogramming with Macros. Also, the function new is not really a special constructor function (if you are familiar with languages with constructors), but just a conventional name adopted by the community as a method name to create new instances of types.

Now, before we get into more complex trait implementations, we need to auto-derive some more built-in traits that will help us implement more high-level functionality. Let's look at some of them:

- Debug: We have already seen this before. As the name suggests, this trait helps types to be printed on the console for debugging purposes. In the case of a composite type, the types will be printed in a JSON-like format with braces and parentheses, and quotes if the type is a string. This is implemented for most built-in types in Rust.

- PartialEq and Eq: These traits allow two items to be compared to each other for equality. For our complex type, only PartialEq makes sense, because when our complex type contains f32 or f64 values, we cannot compare them since Eq is not implemented for f32 and f64 values. PartialEq defines partial ordering. whereas Eq requires a total ordering, Total ordering is undefined for floats, as NaN is not equal to NaN.  NaN is a type in floating point types that represents an operation whose result is undefined, such as 0.0 / 0.0.

- Copy and Clone: These traits define how types get duplicated. We have a separate section for them in Chapter 6, Memory Management and Safety. In brief, when auto-derived on any custom type, these traits allow you to create a new copy from the instance, either implicitly when Copy is implemented or explicitly by calling clone() on them when Clone is implemented. Please note that the Copy trait depends on Clone being implemented on types.

Trait objects are Rust's way of performing dynamic dispatch where we don't have the actual concrete type information. Method resolution is done by hopping down to the vtable and invoking the appropriate method. One of the use cases for trait objects is that they allow you to operate on a collection that can have multiple types, but with an extra pointer indirection at runtime. To illustrate this, consider the following program:

# Memory Management and Safety
> Memory management is a fundamental concept to understand for anyone working with a low-level programming language.

Low-level languages don't come with automatic memory reclamation solutions like a built-in garbage collector, and it's the responsibility of the programmer to manage memory that's used by the program. 

Having knowledge of where and how memory gets used in a program enables programmers to build efficient and safe software systems. A lot of bugs in low-level software are due to improper handling of memory. At times, it's the programmer's mistake. The other times, it's the side effect of the programming language used, such as C and C++, which are infamous for a lot of memory vulnerability reports in software. Rust offers a better, compile-time solution to memory management. It makes it hard to write software that leaks memory unless you explicitly intend to! Programmers who have done a fair amount of development with Rust eventually come to the realization that it discourages bad programming practices and directs the programmer toward writing software that uses memory safely and efficiently.

- Programs and memory
> As a motivation to understand memory and its management, it's important for us to have a general idea of how programs are run by the operating system and what mechanisms are in place that allow it to use memory for its requirements.

Every program needs memory to run, whether it's your favorite command-line tool or a complex stream processing service, and they have vastly different memory requirements. In major operating system implementations, a program in execution is implemented as a process. A process is a running instance of a program. When we execute ./my_program in a shell in Linux or double-click on my_program.exe on Windows, the OS loads my_program as a process in memory and starts executing it, along with other processes, giving it a share of CPU and memory. It assigns the process with its own virtual address space, which is distinct from the virtual address space of other processes and has its own view of memory.

During the lifetime of a process, it uses many system resources. First, it needs memory to store its own instructions, then it needs space for resources that are demanded at runtime during instruction execution, then it needs a way to keep track of function calls, any local variables, and the address to return to after the last invoked function. Some of these memory requirements can be decided ahead at compile time, like storing a primitive type in a variable, while others can only be satisfied at runtime, like creating a dynamic data type such as Vec<String>. Due to the various tiers of memory requirements, and also for security purposes, a process's view of memory is divided into regions known as the memory layout.

# Trifecta of memory safety
> The concepts that we will explore next are the core tenets of Rust's memory safety and its zero cost abstraction principle. They enable Rust to detect memory safety violations in a program at compile time, provide automatic freeing of resources when their scope ends, and much more. We call these concepts ownership, borrowing, and lifetimes. Ownership is kind of like the core principle, while borrowing and lifetimes are type system extensions to the language, enforcing and sometimes relaxing the ownership principle in different contexts in code to ensure compile-time memory management. Let's elaborate on these ideas.

# Ownership
> The notion of a true owner of a resource in a program differs across languages. Here, by resource, we collectively refer to any variable holding a value on the heap or the stack, or a variable holding an open file descriptor, a database connection socket, a network socket, and similar things. All of them occupy some memory from the time they exist until the time they are done being used by the program. An important responsibility of being the owner of a resource is to judiciously free the memory used by them, as not being able to perform deallocations at proper places and times can lead to memory leaks.

# Borrowing
> The concept of borrowing is there to circumvent the restrictions with the ownership rule. Under borrowing, you don't take ownership of values, but only lend data for as long as you need. This is achieved by borrowing values, that is, taking a reference to a value. To borrow a value, we put the  & operator before the variable & is the address of operator . We can borrow values in Rust in two ways.
