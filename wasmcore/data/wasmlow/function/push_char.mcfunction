# Params: charcode, rets.str
# Push a char to rets.str

data modify storage wasm:c args.str0 set from storage wasm:c rets.str
$data modify storage wasm:c args.str1 set from storage wasm:chars list[$(charcode)]
function wasmlow:string_cat with storage wasm:c args