importScripts('/super_floppy.js');
globalThis.module = { exports: {} };
importScripts('/comlink-worker.js');
const { Comlink } = module.exports;

const main = Comlink.wrap(self);

(async () => {
    const binary = await fetch('/super_floppy_bg.wasm');
    await wasm_bindgen({ module_or_path: binary });
    const { SolverWrap } = wasm_bindgen;
    const solver = new SolverWrap();

    const table = await (await fetch('/prune.table')).arrayBuffer();
    solver.set_pruning_table(new Uint8Array(table));

    Comlink.expose(solver);

    await main.onWorkerReady();
})();
