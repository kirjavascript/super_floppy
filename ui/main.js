import './style.css';
import './src/renderer';
import * as Comlink from 'comlink';

const worker = new Worker(new URL('./src/worker.js', import.meta.url))
const solver = Comlink.wrap(worker);
Comlink.expose({ onWorkerReady }, worker);

async function onWorkerReady() {
    console.log(await solver.get_state());
}
