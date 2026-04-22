$execute store result score l$(i) wasm0 run data get storage wasm:s temp.args[$(i)]
scoreboard players add i wasm 1
execute store result storage wasm:s temp.i int 1 run scoreboard players get i wasm
execute if score i wasm < args_len wasm run function wasmlow:copy_args with storage wasm:s temp