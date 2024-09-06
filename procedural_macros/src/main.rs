use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Custom derive macros, only work on structs and enums
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// Attribute-like macros
// #[route(GET, "/")]
// fn index() {
//     // --snip--
// }

// #[proc_macro_attribute]
// pub fn route(
//     attr: TokenStream, // GET, "/"
//     item: TokenStream, // fn index() { ... }
// ) -> TokenStream {
//     // --snip--
// }


// Function-like macros

let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // --snip--
}