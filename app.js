import "bootstrap";

import init, { run } from "./pkg/app.js";

async function main() {
    await init("/pkg/app_bg.wasm");
    run();
}

main();
