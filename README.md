# Toyrobot by a beginner Rust user

This is an implementation of the toy robot challenge in Rust.

I have only read Rust By Example and scanned through the official Rust book.

I would have found the syntax very challenging to get right quickly without Github Copilot.

Despite all the static checking, I still found a couple of errors with unit tests:

-   `usize` converting to `i32` resulted in movement off the bottom wrapping to the other side
-   parsing didn't allow for commas, required spaces

I really enjoyed the process of starting with `unwrap` everywhere and then replacing it with proper error handling. I've never written such robust code before, it was eye-opening.

Cargo is a truly wonderful tool. It works so unbelievably well, far better than `pip` or `npm`, and unbelievably better than `maven` or c/c++'s complete lack of a package manager.

The unit tests were painless.

I was worried that probably glossed over a few copy/ref performance disoptimizations due to relying heavily on Copilot, but I think that without using the `#derive(Copy)` then there shouldn't be any problems.

I had one minor issue with the borrow checker but resolved it in a couple of minutes.
