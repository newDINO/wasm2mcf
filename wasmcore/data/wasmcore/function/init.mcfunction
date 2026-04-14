data remove storage wasm:c stack
data remove storage wasm:c locals
data remove storage wasm:c args
data remove storage wasm:c args1
data remove storage wasm:c rets

# "args", "args1" and "rets" are for calling mcfunctions, wasm function arguments are passed using locals
# "rets" are for returning strings and objects
data merge storage wasm:c {stack: [], locals: [], args: {}, args1: {}, rets: {}}
data merge storage wasm:chars {list: [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', ' ']}

scoreboard objectives remove wasm
scoreboard objectives add wasm dummy "wasm"
scoreboard players set one wasm 1
scoreboard players set local_index wasm -1