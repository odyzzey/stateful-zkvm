# Stateful zkVM Experiment

This repo explores exposing an application's read-only state to a usually stateless zkVM runtime.

The host code generates a random list of numbers to be sorted over multiple runs of the guest program through the zkVM.
The guest code implements a naive sorting algorithm that requests state from the host machine via a callback, then outputs instructions for the state transition function. These transitions are applied post-verification, and eventually sort our state from least to greatest.

An example output:

```
$ cargo run

Initial    [2, 0, 9, 3, 6, 8, 4, 1, 7, 5]
State 0    [0, 2, 3, 9, 6, 4, 8, 1, 5, 7]
State 1    [0, 2, 3, 6, 9, 4, 1, 8, 5, 7]
State 2    [0, 2, 3, 6, 4, 9, 1, 5, 8, 7]
State 3    [0, 2, 3, 4, 6, 1, 9, 5, 7, 8]
State 4    [0, 2, 3, 4, 1, 6, 5, 9, 7, 8]
State 5    [0, 2, 3, 1, 4, 5, 6, 7, 9, 8]
State 6    [0, 2, 1, 3, 4, 5, 6, 7, 8, 9]
State 7    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```
## Caveats
Some things to consider for potential implementations
- zkVM inputs are private, so something in the output should ensure a malicious host can't feed an invalid state
- When two proofs that permutate the same state run in parallel and submit for verification back-to-back, under what conditions should the later submission be invalidated?
    - What if the new state wouldn't necessarily change the output of the second submission? Should the have to regenerate a new proof?
    - In a module-based app architecture, would one module's proof being too late invalidate every other one in the application?
