use std::{collections::HashMap, fmt::Write, fs, vec};

use serde::Serialize;
use wasmparser as wp;

const DONT_SUPPORT: &str = "Doesn't support yet";

fn main() {
    let src_path = "target/wasm32-unknown-unknown/release/wasmtest.wasm";
    let data = fs::read(src_path).unwrap();
    let parser = wp::Parser::new(0);
    let payloads = parser.parse_all(&data);

    let mut types = Vec::new();
    let mut function_type_ids = Vec::new();

    let mut func_exports: HashMap<u32, String> = HashMap::new();

    let target_path = "target/testpack/data/testpack/function";

    let mut code_section_index = 0;

    for payload in payloads {
        let payload = payload.unwrap();
        match payload {
            wp::Payload::TypeSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let rectype = item.unwrap().1;
                    for subtype in rectype.into_types() {
                        types.push(subtype.composite_type.inner);
                    }
                }
                println!("{:?}", types);
            }
            wp::Payload::FunctionSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let index = item.unwrap().1;
                    function_type_ids.push(index);
                }
                println!("{:?}", function_type_ids);
            }
            wp::Payload::MemorySection(r) => {
                for item in r.into_iter_with_offsets() {
                    let mem = item.unwrap().1;
                    let page_size = mem.page_size();
                    println!("Memory initial size: {}", page_size * mem.initial as u32);
                }
            }
            wp::Payload::ExportSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let export = item.unwrap().1;
                    match export.kind {
                        wp::ExternalKind::Func => {
                            func_exports.insert(export.index, export.name.to_owned());
                        }
                        _ => {}
                    }
                }
            }
            wp::Payload::CodeSectionEntry(r) => {
                let mut ro = r.get_operators_reader().unwrap();
                let mut mcf = String::new();
                writeln!(&mut mcf, "function wasmcore:func_enter").unwrap();
                while let Ok(operator) = ro.read() {
                    transpile(operator, &mut mcf);
                }
                if let Some(export_name) = func_exports.get(&code_section_index) {
                    fs::write(
                        format!("{}/{}.mcfunction", target_path, export_name),
                        mcf.as_bytes(),
                    )
                    .unwrap();
                }
                code_section_index += 1;
            }
            wp::Payload::DataSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let data = item.unwrap().1;
                    match &data.kind {
                        wp::DataKind::Active {
                            memory_index,
                            offset_expr,
                        } => {
                            let offset = offset_expr.get_operators_reader().read().unwrap();
                            let wp::Operator::I32Const { value: offset } = offset else {
                                panic!("{}", DONT_SUPPORT);
                            };
                            println!("memory: {}, offsewasmcore: {}", memory_index, offset);
                        }
                        wp::DataKind::Passive => {
                            panic!("{}", DONT_SUPPORT);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let command_storage = CommandStorage {
        data: CommandStorageData {
            contents: CommandStorageContents {
                m: MemData {
                    mems: crab_nbt::NbtTag::List(vec![crab_nbt::NbtTag::IntArray(vec![0, 1, 2])]),
                },
            },
        },
        data_version: 0,
    };
    let command_storage_file = fs::File::create("target/command_storage.dat").unwrap();
    crab_nbt::serde::ser::to_writer(&command_storage, "".to_owned(), command_storage_file).unwrap();
}

#[derive(Serialize)]
struct CommandStorage {
    data: CommandStorageData,
    #[serde(rename = "DataVersion")]
    data_version: i32,
}
#[derive(Serialize)]
struct CommandStorageData {
    contents: CommandStorageContents,
}

#[derive(Serialize)]
struct CommandStorageContents {
    m: MemData,
}

#[derive(Serialize)]
struct MemData {
    mems: crab_nbt::NbtTag,
}

fn transpile(operator: wp::Operator, out: &mut String) {
    match operator {
        wp::Operator::LocalGet { local_index } => {
            writeln!(out, "function wasmcore:local_get {{a0: {}}}", local_index).unwrap()
        }
        wp::Operator::I32Add => writeln!(out, "function wasmcore:add").unwrap(),
        wp::Operator::I32Mul => writeln!(out, "function wasmcore:mul").unwrap(),
        wp::Operator::End => writeln!(out, "function wasmcore:func_ret").unwrap(),
        _ => panic!("Doesn't support operator: {:?} yet.", operator),
    };
}
