// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum GenericVar<'a> {
    Int(u32),
    Str(&'a str),
    Bool(bool),
    Byte(u8),
}

fn main() {
    let mut shopping_list: Vec<GenericVar> = Vec::new();
    shopping_list.push(GenericVar::Str("milk"));
}
