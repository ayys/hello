fn main() {
    // read message.txt into the program
    let message = "Hello from Wasmer ❤️
===

This message shows that your installation appears to be working correctly.

To try something more fun, you can run cowsay with:
 $ echo Moo | wasmer run cowsay

Publish packages, run apps, and more with a free Wasmer account:
 https://wasmer.io/

For more examples and ideas, visit:
 https://wasmer.io/templates/";
    println!("{}", message);
}
