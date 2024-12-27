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
const solution = document.querySelector('#solution');

function addMenuItem(label, callback) {
    solution.textContent = '';
    const button = menu.appendChild(document.createElement('button'));
    button.textContent = label;
    button.addEventListener('click', callback);
}

addMenuItem('solve', async () => {
    solution.textContent = 'solving ...';
    solution.textContent = await solver.solve();
});

addMenuItem('solved state', async () => {
    await solver.set_solved_state();
    updateRenderState(await solver.get_state());
});

addMenuItem('random state', async () => {
    await solver.set_random_state();
    updateRenderState(await solver.get_state());
});

[...'RLUD'].flatMap(move => [move, move+"'", move+'2'])
    .forEach(move => {
        addMenuItem(move, async () => {
            await solver.do_moves(move);
            updateRenderState(await solver.get_state());
        });
    });

const scramble = menu.appendChild(document.createElement('input'));

addMenuItem('do moves', async () => {
    await solver.do_moves(scramble.value.replace(/([RLUD](['2])?)/g, '$1 '));
    updateRenderState(await solver.get_state());
});
