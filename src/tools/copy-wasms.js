const fs = require('fs');
const files = fs.readdirSync('target/wasm32-wasi/debug');

fs.rmSync('bin', {
    recursive: true,
    force: true,
});

fs.mkdirSync('bin');

files.forEach(f => {
    if (f.endsWith('.wasm')) {
        const name = f.replace('.wasm', '');
        console.info(`copying ${name}`);
        fs.mkdirSync(`bin/${name}`);
        fs.copyFileSync(`target/wasm32-wasi/debug/${f}`, `bin/${name}/app.wasm`)
    }
});