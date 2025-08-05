# MultiMake
A build tool designed for automating multiple individual compilations.


# Types
* Strings
* Ints
* Floats
* Booleans
* Lists
* Dictionaries
* Void


# Syntax
```
# You can declare comments with a hash (#)
# You can specify a function with the keyword 'fun' and by giving it a name
# You can specify directories to run the task in square brackets
# By default, the starting function should take in all remaining cmdline args as a list
# You can specify function task outputs in comma separated square brackets
# Everything int the body is ran once except for things in a task block
fun build[src](args) [build,log] {
    # variables are accessed with a prefixed dollar ($)
    # you can unpack lists
    [one, two, ..rest] = $args
    
    # there are conditionals
    if $one == $two {
        # functions can be called with parenthesis
        exit("Args must be unique")
    }

    # Commands can be run by simply listing out their name and their arguments
    # These will be run in the user's shell
    mkdir build
    
    # Everything will be run for each specified directory
    # In task blocks, all functions defined in the file will become inaccessible.
    # Instead, functions in .mm files in the specified directories will become available.
    task {
        # this will call the build task in each directory specified with the args passed into this file's build
        build($args) # This will not be a recursive call
    }
} 

fun fail() {
    # tasks fail immediately upon non-zero exit codes of commands or when a value is used incorrectly
    arstarsta

    printf "not printed"
}

fun variable-expansion() {
    # when passed into commands all types will be converted into Strings
    # for lists, they will be expanded as individual arguments to commands
    # to control the expansion of lists, call the expand task and provide a separator as a string

    mkdir ["-p", "build/release"] # becomes: mkdir -p build/release
    some-command expand(["p", "q", "j"], "") # becomes: some-command pqj
}

fun string-evaluation() {
    # You can evaluate MultiMake code in ${} to build new strings
    # It doesn't matter if you use single or double quotes
    
    some-command '-${expand(["p", "q", "j"], "")}' # becomes: some-command -pqj
}

fun loops() {
    # There are only for loops which can iterate over lists
    accum = 0
    for value in range(0, 5, 1) {
        accum = accum + $value
    }
}

fun returning() {
    # you can run any type of value from a function
    return 0
}

fun variable-command-calling() {
    # A single variable expression will be evaluated as a command
    foo = "echo"
    $foo "Hello, World!"
}

```
