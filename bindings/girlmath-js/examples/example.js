const { Format, loadWasmSync } = require("../");

function main() {
    loadWasmSync();
    
    let result = Format.formatCurrencyAmount("USD", 100)
    console.log(result);
}

main();