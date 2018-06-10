# Introductory exercises

1. Create a **[Hello World!](https://doc.rust-lang.org/rust-by-example/hello.html)** program.

   > Use *[comments](https://doc.rust-lang.org/rust-by-example/hello/comment.html)*.
   
   > Use *[formatted prints](https://doc.rust-lang.org/rust-by-example/hello/print.html)*.
   
2. Create a multiplication table printer that uses every **[primitive](https://doc.rust-lang.org/rust-by-example/primitives.html)**.
  
   > Using *[literals and operators](https://doc.rust-lang.org/rust-by-example/primitives/literals.html)*, print a title with the number of the multiplication table.
   
   > Using *[tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)*, create a tuple for every multiplication, with the following format: *(numberA, numberB, product)*.
   
   > Store the tuples with *[arrays and slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)* and print each tuple.
   
3. Create a program to locate dogs and to know what they are doing, using **[Custom Types](https://doc.rust-lang.org/rust-by-example/custom_types.html)**.
   
   > Use *[structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)* to represent Dogs and 2D Positions. Each Dog has a name and a position. The program should print the name and position of every dog it contains.
   
   > Use *[enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)* to represent the Dog's current action. Add a new field to the Dog struct created previously and print the name, position and action of every dog.

   > With *[constants](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html)*, create a variable that stores the max number of stored dogs, that can be changed during runtime. Additionally, create a constant that stores the current version of the program (e.g. V1.00).

4. Create a program that represents a basic Safe Box which stores money, using **[variable bindings](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)**.

   > Using *[mutability](https://doc.rust-lang.org/rust-by-example/variable_bindings/mut.html)*, make the money stored go from zero euros to 1000 euros and finally up to 2 million euros during runtime.
   
   > Using *[scope and shadowing](https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html)*, create a secret Safe Box inside a block with a million euros from the first Safe Box stored, and print its money. Make sure to decrement a million euros from the first Safe Box inside the block.
   
   > By *[declaring first](https://doc.rust-lang.org/rust-by-example/variable_bindings/declare.html)* a variable named *secret_safe_box_created*, initialize it inside the previously coded block with the value *true*.
   
5. Create a program and do the following exercises, using **[several mechanisms to change or define the type of primitive and user defined types](https://doc.rust-lang.org/rust-by-example/types.html)**.

   > Use *[casting](https://doc.rust-lang.org/rust-by-example/types/cast.html)* to code a program that converts a floating number to a char. Print *Hello World!* using this method, for each char.

   > Using the same methods, with the help of *[suffixes](https://doc.rust-lang.org/rust-by-example/types/literals.html)*, print *Hello World!*, in which the word *Hello* uses casting from type *u8* and the remaining characters of the string use casting from type *u32* but with values higher than 256.
   
   > To understand *[inference](https://doc.rust-lang.org/rust-by-example/types/inference.html)*, create a empty vector and try to push a integer and then a string. See what happens and try to discover why.
   
   > Using *[aliasing](https://doc.rust-lang.org/rust-by-example/types/alias.html)*, create a new name for *u64* called *WayBigInteger* and print it with the value 30072017.
