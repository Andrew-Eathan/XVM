# XVM  
first rust project :D  
xvm is a custom processor i originally wrote in C++ that i ported to rust  
  
xvm features:  
- 16 registers with instructions to load ints from 8 to 64 bits  
- arithmetic, binary and special instructions
- memory you can push bytes to  
- a stack for you to use, currently only takes 64 bit ints until i figure out how to make it support floats too  
- interrupts you can call and attach interrupt listeners for
- ability to dump the stack/registers in the command line for debugging  
  
and will later feature:
- graphics  
- reading memory from files/writing to files instead of pushing bytes  
and more when i remember to add stuff