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

I set it up to have some random numbers for A and B (there is no checking on accuracy atm, I want to add in a python layer to check the output, kinda just manual atm). C doesnt matter because that is the write to location

