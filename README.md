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

The data is a 1d array that gets seperated into a 2d array by comb_mem_d2(). This provides compile time checking to make sure the data entries fall within the deinfed range from the width and is_signed (ie -1 fails)

I set it up to have some random numbers for A and B (there is no checking on accuracy atm, I want to add in a python layer to check the output, kinda just manual atm, need to look into the .expect from repo). C should not matter because it is the write location. I would need some code to init it to zero at the start or maybe just use accum and set it to zero.

External keyword connects the declared memory to the memory made in the data.json

Would have been cool to choose size values that are 2^n so that the counters reset automatically

Could check if register is never used, had acc that was not used

Check if the multiplciation pipeline is used without break (ie the compiler is smart enough to always put in another value)

Enjoy the strict compile time check for bit sizes