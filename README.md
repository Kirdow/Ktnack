# Ktnack
Stack-based compiled language made in Rust. This project is made for me to learn Rust coming from C++.

## Build, Compile & Run
Uses only core cargo crates to my knowledge.<br>
How to build and use:
1. Build the Ktnack compiler using cargo:
```sh
cargo build
```
2. Compile your program (using `code.ktnack` as example).
```sh
target\debug\ktanck.exe code.ktnck
```
You can also use the `build.bat` file by typing `build.bat code.ktnck`.<br>
3. Run your compiled program.
```sh
code.exe
```

## References
Inspired by [Porth](https://gitlab.com/tsoding/porth) by [Tsoding](https://www.youtube.com/@TsodingDaily).

# Feature set
Ktnack offers a wide variety of keywords and features, here you'll find the general idea of what you can do with the language.

## Printing an integer value
This prints the value `12` onto the terminal:
```
12 .
```
Here we first push `12` onto the stack.<br>
Then we use `.` which pops 1 i64 off the stack and prints it.

## Arithmetic operators
These few examples show how to add, subtract, multiply, divide and modulo
```
7 5 +
7 5 -
7 5 *
7 5 /
7 5 %
```
Add `.` operator at the end of each line to print the results.<br>
Taking for example `7 5 -`, this is equivalent to `7 - 5`,<br>
it pops 7 and 5 off the stack, subtract 5 from 7, then pushes `2` back onto stack.

## Logical operators
The logical operators include `<`, `>`, `<=` `>=`, `=` and `!=`.<br>
Each one pops two values off the stack, and compares the top most item<br>
as the right hand side with the second from top item as the left,<br>
then pushes `1` if it's true, or `0` if it's false.

Here's an example:
```
5 7 < .
5 7 > .
```
The output of this would be:
```
1
0
```

## If statement
If statements pop one value off the stack, and if the value<br>
is 0 it jumps past `else` if it has one, otherwise to `end`.<br>
If it's any other value, it runs its own block, and once it<br>
reaches `else` it jumps to `end`.<br>

Here's some dummy code:
```
1 if
    1 .
else
    2 .
end
```
The output of this would be `1`.<br>
Take this code:
```
0 if
    1 .
end
```
This wouldn't have any output, as the value was `0`, thus it jumped past `end`.<br>
As you might have guessed, this works with logical operators:
```
5 7 < if
    5
else
    7
end
```
As you can see this roughly translates to a `min` ternary in other languages,<br>
which would look like `5 < 4 ? 5 : 4`.

## Dup, over, swap and drop
These are 4 operators which manipulate the stack directly.

### Dup
`dup` will pop one value off the stack and push it back twice.
```
5 7 dup . . .
```
This will output
```
7 7 5
```

### over
`over` takes the second value on the stack and copies it to the top.
```
5 7 over . . .
```
This will output
```
5 7 5
```

### swap
`swap` takes the top two values on the stack and switch their places.
```
5 7 swap . .
```
This will output
```
5 7
```

### drop
`drop` will pop the top value off the stack and discard it.<br>
Basically cleans up no longer needed values off the stack.<br>
You will see this a lot with loops.
```
5 7 drop .
```
This will output
```
5
```

## While loop
While loops start with `while`, followed by the code which acts as the condition point.<br>
Being stack based, there's no limit on the size to this. This section ends with `do`<br>
which is where the loop's body is. The loop itself ends with `end`, which will<br>
jump back to `while`, where the condition is checked.

The condition exactly like `if`, where if the value is `0` it jumps past `end`,<br>
otherwise it runs the loop's body.

Here's an example of a loop iterating from 0 to 9
```
0 while dup 10 < do
    dup .
    1 +
end
drop
```
This first sets the starting point to `0`.<br>
Now we make the condition `is the value less than 10`.<br>
Since we consume the values with any operation, we need to clone it,<br>
which is where `dup` comes in. Then inside the loop, we once again<br>
clone the value in order to print it. Then we add 1 to the value.<br>
Lastly after the loop we use `drop` as we won't need the value anymore.<br>
Without the use of `drop` the stack would be misaligned, so we drop it.<br>

## Memory access
You also got access to a buffer of 640,000 bytes.<br>
This is accessed using two functions, `S` for save and `L` for load.<br>
These allow you to save single bytes at various locations.<br>

You get the buffer address using `@`, and to get an exact address<br>
of for example `5` you'd do `@ 5 +`.

Save takes `value address S`.
```
12 @ 6 + S
```
This saves the value `12` at index `6` in the buffer.

Load takes `address L`.
```
@ 3 + L
```
This loads from index `3` in the buffer onto the stack.

## Printing strings
Accessing the memory allows you to push utf-8 values onto the memory,<br>
which you can then print using `P` or `p`.<br>
`P` places a new line at the end of the string, while<br>
`p` prints without a new line.

Printing takes `address count P`.<br>
```
65 @ 0 + S
66 @ 1 + S
67 @ 2 + S
@ 0 + 3 P
```
This prints `ABC` with a new line.

## String literals
String literals are strings defined in the source code using quotes.<br>
Using string literals pushes its address and character count onto the stack,<br>
in a way where it's ready to print.

String literals are located at its own piece of the executable just like with<br>
other compiled languages, and should thus not be written to.

You can define and print a string literal like this:
```
"Hello, World!" P
```
*Woo finally hello world!*<br>
This prints what you'd expect.
