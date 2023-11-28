const { format_currency_amount_wasm, loadWasmSync } = require("../");

function main() {
    loadWasmSync();
    
    let result = format_currency_amount_wasm("USD", 100)
    console.log(result);
}

main();