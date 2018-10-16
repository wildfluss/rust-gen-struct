extern crate clang;

use clang::*;
use std::os::raw::{c_ushort, c_uint, c_uchar, c_int, c_void};

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

    println!("use std::os::raw::{{c_ushort, c_uint, c_uchar, c_int, c_void, c_char}};");

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
            None => (),
        };
    }

    Ok(())
}

fn print_struct(name: String, type_: Type, struct_: Entity) {
    println!("#[repr(C)]");
    println!("pub struct {} {{", name);
    // match type_.get_sizeof() {
    //     Ok(size) => println!(" (size: {} bytes)", size),
    //     Err(error) => println!(" get_sizeof: {:?}", error),
    // };

    for field in struct_.get_children() {
        match field.get_name() {
            Some(name) => {
                print!("    {}", if name == "type" { "type_".to_string() } else { name });
                let rs_type = match field.get_type() {
                    Some(type_) =>
                        match type_.get_kind() {
                            // XXX
                            clang::TypeKind::UShort => "c_ushort".to_string(),
                            clang::TypeKind::UInt => "c_uint".to_string(),
                            clang::TypeKind::UChar => "c_uchar".to_string(),
                            clang::TypeKind::Int => "c_int".to_string(),
                            clang::TypeKind::Pointer => {
                                if type_.get_display_name() == "const char *" {
                                    "*const c_char".to_string()
                                } else {
                                    "*mut c_void".to_string()
                                }
                            },
                            _ => format!("{:?}", type_),
                        },
                    None => "None".to_string()
                };
                println!(": {},", rs_type);
                // match type_.get_offsetof(&name) {
                //     Ok(offset) => println!(" (offset: {} bits)", offset),
                //     Err(error) => println!(" get_offsetof: {:?}", error),
                // }
            }
            None => println!("    None"),
        };
    }
    println!("}}");
}
