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
                let ro = r.get_operators_reader().unwrap();
                let mut mcf = String::new();
                transpile(ro.into_iter(), &mut mcf, &types, &funcs_info, &pack_name);
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
    operators: wp::OperatorsIterator,
    out: &mut String,
    types: &[wp::CompositeInnerType],
    funcs_info: &[FuncInfo],
    pack_name: &str,
) {
    let mut stack_len = 0;

    for operator in operators {
        let operator = operator.unwrap();
        match operator {
            wp::Operator::Nop => {}
            wp::Operator::End => {}
            wp::Operator::Call { function_index } => {
                let info = &funcs_info[function_index as usize];

                let wp::CompositeInnerType::Func(func_type) = &types[info.ty as usize] else {
                    panic!("The type of the function is not of FuncType");
                };

                writeln!(out, "scoreboard players add si wasm 1").unwrap();
                writeln!(out, "scoreboard players add si1 wasm 1").unwrap();
                writeln!(out, "execute store result storage wasm:s call_args.si int 1 run scoreboard players get si wasm").unwrap();
                writeln!(out, "execute store result storage wasm:s call_args.si1 int 1 run scoreboard players get si1 wasm").unwrap();
                writeln!(
                    out,
                    "$scoreboard objectives add wasm$(si1) dummy 'wasm$(si1)'"
                )
                .unwrap();
                for i in 0..func_type.params().len() {
                    let param = func_type.params()[i];
                    match param {
                        wp::ValType::I32 => {
                            writeln!(
                                out,
                                "$execute store result score l{} wasm$(si1) run scoreboard players get v{} wasm$(si)",
                                i, stack_len - func_type.params().len() + i
                            ).unwrap();
                        }
                        _ => panic!("Don't support function parameter types other than I32"),
                    }
                }
                let path = if info.is_import {
                    "wasmhigh"
                } else {
                    pack_name
                };
                writeln!(
                    out,
                    "function {}:{} with storage wasm:s call_args",
                    path,
                    info.name.unwrap()
                )
                .unwrap();
                for i in 0..func_type.results().len() {
                    let ret = func_type.results()[i];
                    match ret {
                        wp::ValType::I32 => {
                            writeln!(out, "$execute store result v{} wasm$(si) run scoreboard players get v{} wasm$(si1)", stack_len + i, i).unwrap();
                        }
                        _ => panic!("Don't support function return types other than I32"),
                    }
                }
                writeln!(out, "$scoreboard objectives remove wasm$(si1)").unwrap();
                writeln!(out, "scoreboard players remove si wasm 1").unwrap();
                writeln!(out, "scoreboard players remove si1 wasm 1").unwrap();
                writeln!(out, "execute store result storage wasm:s call_args.si int 1 run scoreboard players get si wasm").unwrap();
                writeln!(out, "execute store result storage wasm:s call_args.si1 int 1 run scoreboard players get si1 wasm").unwrap();

                stack_len += func_type.results().len();
                stack_len -= func_type.params().len();
            }
            wp::Operator::LocalGet { local_index } => {
                writeln!(
                    out,
                    "$execute store result score v{} wasm$(si) run scoreboard players get l{} wasm$(si)",
                    stack_len, local_index,
                )
                .unwrap();
                stack_len += 1;
            }
            wp::Operator::I32Const { value } => {
                writeln!(
                    out,
                    "$scoreboard players set v{} wasm$(si) {}",
                    stack_len, value,
                )
                .unwrap();
                stack_len += 1;
            }
            wp::Operator::I32Add => {
                writeln!(
                    out,
                    "$scoreboard players operation v{} wasm$(si) += v{} wasm$(si)",
                    stack_len - 2,
                    stack_len - 1
                )
                .unwrap();
                stack_len -= 1;
            }
            wp::Operator::I32Mul => {
                writeln!(
                    out,
                    "$scoreboard players operation v{} wasm$(si) *= v{} wasm$(si)",
                    stack_len - 2,
                    stack_len - 1
                )
                .unwrap();
                stack_len -= 1;
            }
            _ => panic!("Doesn't support operator: {:?} yet.", operator),
        };
    }
}
