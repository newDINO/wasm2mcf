function wasmcore:stack_pop {a0: "v0"}
function wasmcore:stack_pop {a0: "v1"}
scoreboard players operation v0 wasm *= v1 wasm
execute store result storage wasm:c args.a0 int 1 run scoreboard players get v0 wasm
function wasmcore:stack_push with storage wasm:c args