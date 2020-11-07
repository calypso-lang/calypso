# Hello, world!

No programming language guide would be complete without a "Hello, world!" example.

Code in Calypso is organized in "statements". These are complete lines of code ending with a semicolon.
To print text to the console, you call the function `println` with the string of text you would like to print.
To do this, you type the name of the function, followed by an opening parentheses, then the arguments
(in this case, a string of text), then a closing parentheses. Enough talk, let's get to the code!

```calypso
println("Hello, world!");
```

Calypso also allows you to define a main function. This function will always be executed at the beginning of the program.
This turns the interpreter into "item mode", which means that you can no longer execute statements outside of a function.
You can define a main function like this:

```calypso
#[main]
fn main() {
    println("Hello, world!");
}
```

The `#[main]` part of this code, called an attribute, tells the compiler that this is the main function. If we have a
function like this, then the following code will be invalid:

```calypso
#[main]
fn main() {
    println("Hello, world!");
}

println("OOPS! This part is invalid code!");
```

We'll talk about functions, attributes, and all that stuff in more depth in later chapters.
