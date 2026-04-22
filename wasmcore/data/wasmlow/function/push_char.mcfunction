# Params: $(charcode), temp.str
# Push a char to temp.str

data modify storage wasm:s temp.str0 set from storage wasm:s temp.str
$data modify storage wasm:s temp.str1 set from storage wasm:chars list[$(charcode)]
function wasmlow:string_cat with storage wasm:s temp