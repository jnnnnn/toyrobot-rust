# Toyrobot by a beginner Rust user

This is an implementation of [the toy robot challenge](https://joneaves.wordpress.com/2014/07/21/toy-robot-coding-test/) in Rust. For an example spec, see, for example, [a ruby implementation](https://github.com/RafaelChefe/toy_robot).

I am a beginner Rust user, although I am expert in C++. I have only read Rust By Example and scanned through the official Rust book.

I would have found Rust syntax very challenging to get right quickly without Github Copilot.

Despite all the static checking, I still found a couple of errors with unit tests:

-   `usize` converting to `i32` resulted in movement off the bottom wrapping to the other side
-   parsing didn't allow for commas, required spaces

I really enjoyed the process of starting with `unwrap` everywhere and then replacing it with proper error handling. I've never written such robust code before, it was eye-opening.

Cargo is a truly wonderful tool. It works so unbelievably well, far better than `pip` or `npm`, and unbelievably better than `maven` or c/c++'s complete lack of a package manager.

The unit tests were painless (with Copilot).

I was worried that probably glossed over a few copy/ref performance disoptimizations due to relying heavily on Copilot, but I think that without using the `#derive(Copy)` then there shouldn't be any problems.

I had one minor issue with the borrow checker but resolved it in a couple of minutes.

I think I prefer Rust over Golang because Go has ugly error handling and nulls everywhere.

I have this weird idea that Go builds are much faster but I'm not so sure that's true any more. The Go module system is a little more fragile, Cargo is rock solid.
