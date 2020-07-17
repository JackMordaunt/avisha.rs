use code-insiders as editor;
use http-server;
use rollup;
use watchexec;

editor . 
http-server .
watchexec --clear --exts css,html,rs "wasm-pack build --target web && rollup ./main.js --format iife --file ./pkg/bundle.js" 