extern crate clang;

use clang::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} filename struct", args[0]);
        std::process::exit(1);
    }

    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap_or_else(|error| {
        panic!("Clang::new: {:?}", error);
    });

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index
        .parser(&args[1])
        .parse()
        .unwrap_or_else(|source_error| {
            panic!("parse: {:?}", source_error);
        });

    // Get the structs in this translation unit
    let structs = tu
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|e| e.get_kind() == EntityKind::StructDecl)
        .collect::<Vec<_>>();

    // Print information about the structs
    for struct_ in structs {
        let type_ = struct_.get_type().unwrap_or_else(|| {
            panic!("get_type");
        });
        match struct_.get_name() {
            Some(name) => {
                if args[2] == name {
                    print_struct(name, type_, struct_)
                }
            }
            None => print!("struct: None"),
        };
    }

    Ok(())
}

fn print_struct(name: String, type_: Type, struct_: Entity) {
    print!("struct: {:?}", name);
    match type_.get_sizeof() {
        Ok(size) => println!(" (size: {} bytes)", size),
        Err(error) => println!(" get_sizeof: {:?}", error),
    };

    for field in struct_.get_children() {
        match field.get_name() {
            Some(name) => {
                print!("    field: {:?}", name);
                match type_.get_offsetof(&name) {
                    Ok(offset) => println!(" (offset: {} bits)", offset),
                    Err(error) => println!(" get_offsetof: {:?}", error),
                }
            }
            None => println!("    field.get_name: None"),
        };
    }
}
