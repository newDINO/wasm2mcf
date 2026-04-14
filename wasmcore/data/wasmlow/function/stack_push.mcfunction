# Push a value to stack from scoreboard

$execute store result storage wasm:c temp int 1 run scoreboard players get $(stack_push_src) wasm
data modify storage wasm:c stack append from storage wasm:c temp