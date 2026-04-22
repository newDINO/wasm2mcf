# For user to call wasm function. Wasm function should not use this as it start the call stack from 0.

scoreboard objectives add wasm0 dummy "wasm0"

$data merge storage wasm:s {temp: {args: $(args)}}
execute store result score args_len wasm run data get storage wasm:s temp.args
scoreboard players set i wasm 0
execute if score i wasm < args_len wasm run function wasmlow:copy_args { i: 0 }

data merge storage wasm:s {call_args: {si: 0, si1: 1}}
scoreboard players set si wasm 0
scoreboard players set si1 wasm 1
$function $(func) with storage wasm:s call_args