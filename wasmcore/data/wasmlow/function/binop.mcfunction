function wasmlow:stack_pop {stack_pop_dst: "v0"}
function wasmlow:stack_pop {stack_pop_dst: "v1"}
$scoreboard players operation v0 wasm $(op)= v1 wasm
function wasmlow:stack_push {stack_push_src: "v0"}