# 1. Get the index of last element in the stack.
execute store result score t0 wasm run data get storage wasm:c stack
scoreboard players operation t0 wasm -= one wasm
execute store result storage wasm:c args.a0 int 1 run scoreboard players get t0 wasm

# 2. Store the last element to target scoreboard.
$execute store result score $(a0) wasm run function wasmcore:stack_get with storage wasm:c args

# 3. Remove the last element from the stack.
function wasmcore:stack_remove with storage wasm:c args