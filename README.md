# rs
My poorly written ls clone to help me learn rust

Not intented to replace ls by any means unless you want a buggy mess of a program


My goals are a very basic, working implementation of ls in rust and then maybe doing some rewrites 
to make it leaner and more suitable as a day-to-day replacement of ls. 


If you want a real ls clone check out "exa" it's much more feature rich and is probably a lot faster too

If you for whatever reason want to use it clone the repo, and navigate to the rs directory from there run 
```
$ cargo build --release
```
And wait for it to compile. The compiled binary will be in rs/target/release/rs and just move it to /usr/bin if you want to 
use it like you use ls
