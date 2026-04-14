use std::{collections::HashSet, fmt::Write, fs};

use bytemuck::cast_slice;
use serde::Serialize;
use wasmparser as wp;

const DONT_SUPPORT: &str = "Doesn't support yet";

fn main() {
    let src_path = "target/wasm32-unknown-unknown/release/wasmtest.wasm";
    let data = fs::read(src_path).unwrap();
    let parser = wp::Parser::new(0);
    let payloads = parser.parse_all(&data);

    let existing_funcs: HashSet<&str> = HashSet::from_iter(["exec"]);

    let mut types = Vec::new();
    let mut import_func_num = 0;
    let mut funcs_info = Vec::new();

    let pack_name = "testpack";
    let target_path = format!("target/{pack_name}/data/{pack_name}/function");
    fs::remove_dir_all(&target_path).unwrap();
    fs::create_dir(&target_path).unwrap();

    let mut memory = Vec::new();

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
            wp::Payload::ImportSection(r) => {
                for item in r.into_imports_with_offsets() {
                    let import = item.unwrap().1;
                    if !existing_funcs.contains(import.name) {
                        panic!("Importing functions that is not provided");
                    }
                    match import.ty {
                        wp::TypeRef::Func(f) => funcs_info.push(FuncInfo {
                            name: Some(import.name),
                            ty: f,
                            is_import: true,
                        }),
                        _ => panic!("Currently only support importing functions"),
                    }
                    println!("{:?}", import);
                }
                import_func_num = funcs_info.len();
            }
            wp::Payload::FunctionSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let index = item.unwrap().1;
                    funcs_info.push(FuncInfo {
                        name: None,
                        ty: index,
                        is_import: false,
                    });
                }
            }
            wp::Payload::MemorySection(r) => {
                for (i, item) in r.into_iter_with_offsets().enumerate() {
                    if i > 0 {
                        panic!("Currently only support one memory.")
                    }
                    let mem = item.unwrap().1;
                    let page_size = mem.page_size();
                    let mem_size = page_size * mem.initial as u32;
                    memory = vec![0i8; mem_size as usize];
                    println!("Memory initial size: {}", mem_size);
                }
            }
            wp::Payload::ExportSection(r) => {
                for item in r.into_iter_with_offsets() {
                    let export = item.unwrap().1;
                    match export.kind {
                        wp::ExternalKind::Func => {
                            funcs_info[export.index as usize].name = Some(export.name);
                        }
                        _ => {}
                    }
                }
            }
            wp::Payload::CodeSectionEntry(r) => {
                let mut ro = r.get_operators_reader().unwrap();
                let mut mcf = String::new();
                while let Ok(operator) = ro.read() {
                    transpile(operator, &mut mcf, &types, &funcs_info, &pack_name);
                }
                if let Some(func_name) = funcs_info[code_section_index + import_func_num].name {
                    fs::write(
                        format!("{}/{}.mcfunction", target_path, func_name),
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
                            if *memory_index != 0 {
                                panic!("Only support memory index 0");
                            }
                            let offset = offset_expr.get_operators_reader().read().unwrap();
                            let wp::Operator::I32Const { value: offset } = offset else {
                                panic!("{}", DONT_SUPPORT);
                            };
                            memory[offset as usize..offset as usize + data.data.len()]
                                .copy_from_slice(cast_slice(data.data));
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
                m0: MemData { data: memory },
            },
        },
        data_version: 0,
    };
    let mut command_storage_file = fs::File::create("target/command_storage.dat").unwrap();
    na_nbt::to_writer_be(&mut command_storage_file, &command_storage).unwrap();
}

struct FuncInfo<'a> {
    name: Option<&'a str>,
    ty: u32,
    is_import: bool,
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
    m0: MemData,
}

#[derive(Serialize)]
struct MemData {
    #[serde(with = "na_nbt::byte_array")]
    data: Vec<i8>,
}

fn transpile(
    operator: wp::Operator,
    out: &mut String,
    types: &[wp::CompositeInnerType],
    funcs_info: &[FuncInfo],
    pack_name: &str,
) {
    match operator {
        wp::Operator::LocalGet { local_index } => writeln!(
            out,
            "data modify storage wasm:c stack append from storage wasm:c locals[-1][{}]",
            local_index
        )
        .unwrap(),
        wp::Operator::I32Add => writeln!(out, "function wasmlow:binop {{op: '+'}}").unwrap(),
        wp::Operator::I32Mul => writeln!(out, "function wasmlow:binop {{op: '*'}}").unwrap(),
        wp::Operator::End => writeln!(out, "data remove storage wasm:c locals[-1]").unwrap(),
        wp::Operator::I32Const { value } => writeln!(
            out,
            "data modify storage wasm:c stack append value {}",
            value
        )
        .unwrap(),
        wp::Operator::Call { function_index } => {
            let info = &funcs_info[function_index as usize];

            let wp::CompositeInnerType::Func(func_type) = &types[info.ty as usize] else {
                panic!("The type of the function is not of FuncType");
            };

            writeln!(out, "data modify storage wasm:c locals append value []").unwrap();
            for i in 0..func_type.params().len() {
                let stack_index = i as i32 - func_type.params().len() as i32;
                writeln!(
                    out,
                    "data modify storage wasm:c locals[-1] append from storage wasm:c stack[{stack_index}]"
                )
                .unwrap();
            }
            for _ in 0..func_type.params().len() {
                writeln!(out, "data remove storage wasm:c stack[-1]").unwrap();
            }

            let name_space = if info.is_import {
                "wasmhigh"
            } else {
                pack_name
            };
            writeln!(out, "function {}:{}", name_space, info.name.unwrap()).unwrap();
        }
        _ => panic!("Doesn't support operator: {:?} yet.", operator),
    };
}
