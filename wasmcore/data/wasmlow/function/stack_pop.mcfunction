# Pop a value from stack to scoreboard

$execute store result score $(stack_pop_dst) wasm run data get storage wasm:c stack[-1]
data remove storage wasm:c stack[-1]