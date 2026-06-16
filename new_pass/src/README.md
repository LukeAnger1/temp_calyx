# Overview

This folder contains a new pass that converts a while loop in a specific pattern to a static repeat.

To check this, a new line was added into the run script to compile the bad_matmul with this pass. The results were that the bad mat mul 564, while the good mat mul took 361. The bad mat mul with this pass in turn took 361.

# Things to try

This is literally an MVP. This pass needs better detection of when not to run. This pass also needs test cases to make sure it is running properly. It was only tested with a non-nested case that had a particular structure.

It would also be good to expand it to more comparisons than the LT cell and check on the left and right for the constant.

Although I don't particularly want to expand this too much. There are a lot of cases, and compilers always have a hard time recognizing this logic. The solution already exists to simply use a static repeat. 

# What I got

Rust is a new language. A lot of the struggle was behind that. One thing I really like was the suggestions for compile-time failure. I had one bug that was fixed by adding a .clone(), which was suggested.
I would want to spend more time learning the mutable and read-only types. There is also a cool borrowing mechanism that is highly protected at compile time, which I found interesting.

One of the things I saw while researching was a technique to reuse registers to reduce area. Made me go into a rabbit hole of reusing registers for different functionality at different clock speeds. 
