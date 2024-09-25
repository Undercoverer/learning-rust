use std::io;

fn main() {
    println!("Guessing game");
    
    println!("Please enter a number:");

    let _apple = 5;                 // IMMUTABLE BY DEFAULT
                                    // `=` binds values to names immediately (I think that's what they mean)
                                    // (like in a lot of languages)
                                    //
    let mut guess = String::new();  // MUST use `mut` to make variables mutable
                                    // `String` is a class representing a growable (dynamically allocated 
                                    // probably) list of UTF-8 encoded characters
                                    // `::` gets associated functions on a type, in this case
                                    // `::new` is a function on `String` which creates a new empty string
                                    //
                                    // The whole line creates an empty and mutable `String` and binds
                                    // it to `guess`

    io::stdin()                         // I would imagine `io` is not a type, so there must be
                                        // some sort of way to associate functions outside of
                                        // types, but I wouldn't know how to yet
                                        //
                                        // `::stdin()` returns an instance of the `Stdin` type which
                                        // is a reference / "handle" to the process standard input
        
        .read_line(&mut guess)          // an "instance method" on `Stdin` which appends newline
                                        // terminated string from stdin onto the supplied mutable String
                                        // reference. The reference needs to be mutable to allow
                                        // modification because references are immutable by
                                        // default. 
                                        //
                                        // `&` and `&mut` are ways (the only ones I know so far) to
                                        // get references to pieces of data

        .expect("Failed to read line"); // Because `read_line()` returns a `Result` (an enum), all possible
                                        // variants need to be handled (if you want to get
                                        // information out of it I suppose)
                                        // 
                                        // `expect()` is an instance method on `Result` which causes the
                                        // program to crash if it is an `Err`
                                        //
                                        // Nothing bad should happen if the result is an Ok 
                                            
                    
    println!("You guessed: {}", guess); // The {} are like in python kind of where you can put
                                        // expressions in, but also leave them empty and write the
                                        // expression as an argument to `println!()`
    
}
