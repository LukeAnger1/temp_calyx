# The run.sh file runs both implementations, there can be a comparison

# They both use the same initial memory defined in the data.json file

This is the initial structre of the memory. Example here from the [docs](https://docs.calyxir.org/tutorial/language-tut.html#a-memory-cell)

```
{
  "mem": {
    "data": [10],
    "format": {
      "numeric_type": "bitnum",
      "is_signed": false,
      "width": 32
    }
  }
}
```

The code requires typing out the bit width for the address. I feel this can already be calculated from the address. Might look into assigning that automatically, setting up a MACRO type deal, or just switching to the high level language MrXML.

# Initial memory setup

I set it up to have some random numbers for A and B (there is no automatic checking on accuracy atm, the instructions said to setup with .expect, but manually checking it looks fine). C in this code matters to initially be set to zero. I dont have logic to reset it atm. That should be added in.

# Changes from bad mat mul to good mat mul

In order to remove the dynamic handshake per the paper (as the operations have set execution length). I added a static to the outer most position in the control flow.

From there the compile time issues pointed me to the next thing to make static.

One important TODO is to make sure I have the correct static for the matrix multiplication

# Things to try
Would have been cool to choose size values that are 2^n so that the counters reset automatically to save some cycles

Check if the multiplication pipeline is used without a break (ie the compiler is smart enough to always put another value into the pipeline)
I think there was something in the docs to view the compilation. Will need to review.

# Ou

Enjoy the strict compile-time check for bit sizes in the calyx code AND in the data.json. Verilog always upset me on silently failing. This alone saved time by just not having to compile to test.

