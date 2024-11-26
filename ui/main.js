import './style.css';
import './src/renderer';
import * as Comlink from 'comlink';

const worker = new Worker(new URL('./src/worker.js', import.meta.url))
Comlink.expose({ onBoot }, worker);
const solver = Comlink.wrap(worker);

async function onBoot() {
    console.log(await solver.get_state());
}
