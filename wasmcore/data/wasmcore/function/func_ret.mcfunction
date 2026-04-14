execute store result storage wasm:c args.a0 int 1 run scoreboard players get local_index wasm
function wasmcore:locals_remove with storage wasm:c args
scoreboard players operation local_index wasm -= one wasm