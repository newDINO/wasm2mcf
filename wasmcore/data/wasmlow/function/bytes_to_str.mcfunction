# Params:
# $(si)
# l0: start addr
# l1: end addr
# temp.str

$execute store result storage wasm:s temp.addr int 1 run scoreboard players get l0 wasm$(si)
execute store result storage wasm:s temp.charcode int 1 run function wasmlow:mem_get with storage wasm:s temp
function wasmlow:push_char with storage wasm:s temp

$scoreboard players add l0 wasm$(si) 1
$execute if score l0 wasm$(si) < l1 wasm$(si) run function wasmlow:bytes_to_str {si: $(si)}