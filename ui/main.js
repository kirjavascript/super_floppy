import './style.css';
import { updateRenderState } from './src/renderer';
import * as Comlink from 'comlink';

const worker = new Worker(new URL('./src/worker.js', import.meta.url))
const solver = Comlink.wrap(worker);
Comlink.expose({ onWorkerReady }, worker);

export async function updateSolverState(state) {
    await solver.set_state(state);
}
async function onWorkerReady() {
    console.info('worker ready');
}

// UI menu

const menu = document.querySelector('#menu');

function addMenuItem(label, callback) {
    const button = menu.appendChild(document.createElement('button'));
    button.textContent = label;
    button.addEventListener('click', callback);
}

addMenuItem('solved state', async () => {
    await solver.set_solved_state();
    updateRenderState(await solver.get_state());
});

addMenuItem('random state', async () => {
    await solver.set_random_state();
    updateRenderState(await solver.get_state());
});

[...'RLUD'].forEach(axis => {
    [...' \'2'].forEach(type => {
        const move = `${axis}${type}`.trim();

        addMenuItem(move, async () => {
            await solver.do_moves(move);
            updateRenderState(await solver.get_state());
        });
    });
});
