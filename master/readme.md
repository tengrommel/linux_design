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
