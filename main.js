import init, { run_app } from './pkg/avisha.js';
async function main() {
   await init('/pkg/avisha_bg.wasm');
   run_app();
}
main()