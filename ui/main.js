import './style.css';
import { updateRenderState } from './src/renderer';
import * as Comlink from 'comlink';

const worker = new Worker(new URL('./src/worker.js', import.meta.url))
const solver = Comlink.wrap(worker);
Comlink.expose({ onWorkerReady }, worker);

async function onWorkerReady() {
    console.info('worker ready');
    updateRenderState(await solver.get_state());
    console.log(await solver.get_state());
}

export async function updateSolverState(state) {
    await solver.set_state(state);
}
