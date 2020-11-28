?# BrrLang
Yes language go brrr.

## Language Spec

### Variables and Assignment 
New variables are created using the new keyword
```
new variable_name;
``` 
This is how a new variable is created, although this will cause a compilation error because brrrLang does not
allow a variable to contain no value. This is to make avoid any sort of 'null' issues.
#### Assignment
```
10 -> new number;
```
This creates a variable called number and assigns it the value of 10. brrrLang is a statically typed language, although
for variables at least, you do not have to specify the type of a variable, but you can if you want, or if the compiler 
can not infer the type.
```
"Hello world" -> new<String> famous_first_words;
```

### Functions
Void functions are defined as follows:
```
() {
    // Fuck it code.
} -> SomeFunction
```
Parameters are specified between the braces, with the code goes between the curly braces and SomeFunction is replaced
with the name of the function.
#### Functions with parameters
```
(name<String>) {
    std::IO::PrintLine(StrFmt("Hello {}", names));
} -> Greet
```

#### Functions with return
```
// Option 1
(a<i32>, b<i32>) {
    a + b
} -> Sum1[i32]

// Option 2
(a<i32>, b<i32>) {
    return a + b;
} -> Sum2[i32]
```

### Imports and Use
Import and Use declerations are used to use code from external dependencies and pull symbols into you namespace
#### Imports
To make external dependencies available to your code, use the @import declaration to import external packages
```
@import "Std"; // Import the std package
```
#### Adding symbols to the namespace of the current module
```
@use Std::IO::[PrintLine, ReadLine]; // Import PrintLine and ReadLine from the IO module in the Std packges  
@use Std::String::StrFmt; // Import 
```

### Files
All files in brrrLang sould have a snake case name and a package declaration using a pascal case version of the files
name. The extension is .bl. Binary packages should contain a main.bl, there should also be a new statement
```
// main.bl
@pkg "Main"

@import "Std";

() {
    
} -> Main
```

## Comments
Comments are denoted with a `//`. e.g.
```
// This is a comment
```
Multiline comments will also be a thing although do not work as of the current version
```
/*
This is a multiline comment
*/
```


## Compiler overview
### Project configuration
- Load project TOML file
- Check dependencies and download required
- Get compiler tool-chain for target
### Tokenization
- Read, tokenize and scan for imports on the initial file specified in toml
- Find, read, tokenize and scan for imports on the imported files recursively
- Find dependencies and read/tokenize
- Pass built data object with all tokens and scope ids and plenty of metadata for things like errors
### Scope Identification
- Find all variables, Events and data and it's scope
- Build a tree of scope ID's and error check for unreferenced data
### Translation
- Line by line, going thorough the code and using the scopes till the only instructions are the bear minium possible
- Translate into LLVM code inside the basic event runtime
### LLVM Compile
- Send to LLVM to be compiled to target
- Let LLVM do everything actually complicated
- Output resulting binary