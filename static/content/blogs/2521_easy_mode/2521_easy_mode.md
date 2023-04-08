# Puting COMP2521 into Easy Mode

Here are some notes on how to make your life in COMP2521 *way* easier.

NOTE: I am not a COMP2521 tutor. I cannot guarantee the accuracy of my advice.
These are not official CSE instructions.
This information is provided AS IS and WITHOUT WARANTY to the fullest extent of the law.

## First Tip: Don't use GCC

Don't use GCC. It is now falling out of favour in industry, with companies mainly
using it for backwards compatibility.

In order of usefulness:

 1. If you can, use `dcc`. It's built to avoid bugs which novice programmers make. Mostly, you can
    drop it in instead of `gcc`, by replacing `gcc` with `dcc` in your Makefile.
 2. If you can't use `dcc`, use `2521 3c`. Again, just replace `gcc` with `2521 3c`. This enables
    many of the safety features of `dcc`, but without the nice output. It's the same technology
    google uses to debug their code, so it should meet your needs.
 3. If you're using `gdb`, compile with `clang -g` (`2521 3c` also works, but may be slower).

Of course, make sure your code compiles under `gcc` before you submit; but the chances that `gcc` breaks
where `dcc` or `3c` doesn't are minimal.

## Second Tip: Use fydb or gdb

`gdb` is a great tool to help you debug, but it's sometimes confusing to use.

`fydb` is a tool which makes `gdb` easier to use. On CSE machines, it's available at `~apps/bin/fydb`.
If you're struggling to debug something, rather than relying on print statements, you should:

 1. Compile your code with `clang -g`
 2. Run `~apps/bin/fydb ./my_program`
 3. Type `r` to start your program (it may take up to 10 seconds to start)
 4. Interact with your program as normal. When it crashes, it should show you where it crashed.
    You can also use Ctrl+C to stop your program at a certain point, and see the state of your variables.
 5. Type `p` to see the values of variables.
 
 THen, once you've got a hypothesis:
 1. Put a comment on the line where you want to stop your program, which says `//break`.
    This tells fydb to stop on that line.
 2. Again, Run `~apps/bin/fydb ./my_program`
 3. Type `r` to start your program (it may take up to 10 seconds to start)
 4. Interact with your program as normal. When it reaches the line with the `//break` comment, it'll stop.
 5. Follow the instructions on the screen to debug your program further.


## Conclusion

Good luck. Let me know (tom@tfpk.dev or t.kunc@unsw.edu.au) if `fydb` breaks.
