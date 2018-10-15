extern crate clang;

use clang::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} filename struct", args[0]);
        std::process::exit(1);
    }

    // Acquire an instance of `Clang`
    let clang = Clang::new().unwrap();

    // Create a new `Index`
    let index = Index::new(&clang, false, false);

    // Parse a source file into a translation unit
    let tu = index.parser(&args[1]).parse().unwrap();

    // Get the structs in this translation unit
    let structs = tu.get_entity().get_children().into_iter().filter(|e| {
        e.get_kind() == EntityKind::StructDecl
    }).collect::<Vec<_>>();

    // Print information about the structs
    for struct_ in structs {
        let type_ =  struct_.get_type().unwrap();
        let size = type_.get_sizeof().unwrap();
        println!("struct: {:?} (size: {} bytes)", struct_.get_name().unwrap(), size);

        for field in struct_.get_children() {
            let name = field.get_name().unwrap();
            let offset = type_.get_offsetof(&name).unwrap();
            println!("    field: {:?} (offset: {} bits)", name, offset);
        }
    }

    Ok(())
}
