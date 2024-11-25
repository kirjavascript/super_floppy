import './style.css';
import Zdog, { TAU } from 'zdog';

/* bindings needed:
 *
 * set_state
 * get_state
 * set_random_state
 * get_solutions
 */

const element = document.querySelector('#zdog');
const ghostEl = document.querySelector('#ghost');

const defaultRotate = {
    x: -0.4,
    y: 0.2,
};

const ghost = new Zdog.Illustration({
    element: ghostEl,
    rotate: defaultRotate,
});

let hasDragged = false;

const illo = new Zdog.Illustration({
    element,
    dragRotate: true,
    rotate: defaultRotate,
    onDragMove() {
        hasDragged = true;
        ghost.rotate = { x: illo.rotate.x, y: illo.rotate.y, z: illo.rotate.z };
    },
});

illo.setMeasuredSize();
ghost.setMeasuredSize();

const cubieSize = 100;
const cubieGap = 10;
const distance = cubieSize + cubieGap;
const colors = [
    '#ffffff',
    '#0045ad',
    '#b90000',
    '#009b48',
    '#ff5900',
    '#ffd500',
];
const quarter = TAU / 4;

const rotations = {
    x: { y: quarter },
    y: { x: quarter },
};

let cubieIndex = 999; // "global" for joey

function addCubie({
    stickers = [],
    translate,
    rotate,
    isInvisible,
    edgeIndex,
    cornerIndex,
}) {
    const anchor = new Zdog.Anchor({ addTo: illo });
    const container = new Zdog.Anchor({
        addTo: anchor,
        translate,
        rotate,
    });

    const cubieColor = isInvisible ? ' hsla(0, 0%, 90%, 0.5)' : 'black';

    const cubie = new Zdog.Box({
        addTo: container,
        width: cubieSize,
        height: cubieSize,
        depth: cubieSize,
        stroke: true,
        color: cubieColor,
    });

    const colorIndex = 10 + 5 * cubieIndex++;

    const ghostPiece = new Zdog.Box({
        addTo: ghost,
        width: cubieSize,
        height: cubieSize,
        depth: cubieSize,
        color: `rgb(${colorIndex}, 0, 0)`,
        translate,
        rotate,
    });

    if (!isInvisible) {
        const stickerOffset = cubieSize / 2 + 1;

        const ud = [
            { color: 0, axis: 'y', offset: -1 },
            { color: 5, axis: 'y', offset: 1 },
        ];

        [...stickers, ...ud].forEach(({ color, offset, axis }) => {
            const sticker = new Zdog.Rect({
                addTo: container,
                width: cubieSize * 0.9,
                height: cubieSize * 0.9,
                stroke: 2,
                fill: true,
                color: colors[color],
                rotate: rotations[axis],
            });

            sticker.translate[axis] += stickerOffset * offset;
        });
    }

    const data = {
        anchor,
        container,
        destroy: () => {
            anchor.remove();
            ghostPiece.remove();
        },
        colorIndex,
        edgeIndex,
        cornerIndex,
        toggled: false,
        isEdge: stickers.length === 3,
        isCenter: stickers.length === 0 && !isInvisible,
        toggle() {
            if (!data.isEdge && !data.isCenter) {
                data.toggled = !data.toggled;

                cubie.color = data.toggled
                    ? 'hsla(270, 100%, 50%, 0.5)'
                    : cubieColor;
            }
        },
    };

    data.isCorner = !data.isEdge && !data.isCenter;

    return data;
}

addCubie({}); //immutable center

// Edge order: B R F L
// Corner order: BL UB BR UR FR UF FL UL DB DR DF DL
const state = {
    edges: [0, 0, 0, 0],
    corners: [
        'BL',
        undefined,
        'BR',
        undefined,
        'FR',
        undefined,
        'FL',
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
    ],
};

const cubies = [];

const edgeDefs = [
    // Edge order: B R F L
    {
        stickers: [
            { color: 1, axis: 'z', offset: -1 },
            { color: 2, axis: 'x', offset: 1 },
            { color: 4, axis: 'x', offset: -1 },
        ],
        translate: { z: -distance },
        edgeIndex: 0,
    },
    {
        stickers: [
            { color: 2, axis: 'x', offset: 1 },
            { color: 3, axis: 'z', offset: 1 },
            { color: 1, axis: 'z', offset: -1 },
        ],
        translate: { x: distance },
        edgeIndex: 1,
    },
    {
        stickers: [
            { color: 2, axis: 'x', offset: 1 },
            { color: 3, axis: 'z', offset: 1 },
            { color: 4, axis: 'x', offset: -1 },
        ],
        translate: { z: distance },
        edgeIndex: 2,
    },
    {
        stickers: [
            { color: 1, axis: 'z', offset: -1 },
            { color: 3, axis: 'z', offset: 1 },
            { color: 4, axis: 'x', offset: -1 },
        ],
        translate: { x: -distance },
        edgeIndex: 3,
    },
];

const cornerDefs = {
    BL: [
        { color: 1, axis: 'z', offset: -1 },
        { color: 4, axis: 'x', offset: -1 },
    ],
    BR: [
        { color: 2, axis: 'x', offset: 1 },
        { color: 1, axis: 'z', offset: -1 },
    ],
    FR: [
        { color: 2, axis: 'x', offset: 1 },
        { color: 3, axis: 'z', offset: 1 },
    ],
    FL: [
        { color: 3, axis: 'z', offset: 1 },
        { color: 4, axis: 'x', offset: -1 },
    ],
};

const cornerPos = [
    // Corner order: BL UB BR UR FR UF FL UL DB DR DF DL
    { x: -distance, z: -distance },
    { y: -distance, z: -distance },
    { x: distance, z: -distance },
    { y: -distance, x: distance },
    { x: distance, z: distance },
    { y: -distance, z: distance },
    { x: -distance, z: distance },
    { y: -distance, x: -distance },
    { y: distance, z: -distance },
    { y: distance, x: distance },
    { y: distance, z: distance },
    { y: distance, x: -distance },
];

// rotations

const rotatePath = '|z|zz|zzxxx|zzxx|xxzzz|xx|xxx|zzz|zzx|xxz|x'.split('|');
const rotateOrigin = {BL:0,BR:2,FR:4,FL:6};

function renderState() {
    // reset
    cubies.forEach((cubie) => cubie.destroy());
    cubieIndex = 0;
    cubies.splice(0, cubies.length);
    // edges
    state.edges.forEach((rotation, edgeIndex) => {
        const edge = edgeDefs[edgeIndex];
        const axis = edgeIndex & 1 ? 'x' : 'z';
        const sign = edgeIndex === 0 || edgeIndex === 3 ? -1 : 1;
        const rotate = { [axis]: (sign * rotation * TAU) / 4 };

        cubies.push(
            addCubie(
                Object.assign({}, edge, {
                    rotate,
                }),
            ),
        );
    });
    // corners
    state.corners.forEach((corner, cornerIndex) => {
        const obj = corner
            ? { stickers: cornerDefs[corner] }
            : { isInvisible: true };

        obj.translate = {
            ...cornerPos[cornerIndex],
        };

        const originIndex = rotateOrigin[corner];

        if (originIndex !== undefined) {
            let rotations = rotatePath[originIndex] + rotatePath[cornerIndex];
            const rotate = { z: 0, x: 0 };
            if ((corner === 'FR' || corner === 'FL') && [1, 5, 8, 10].includes(cornerIndex)) {
                rotations += 'zz';
            }
            [...rotations].forEach(ch => { rotate[ch]++ });
            rotate.z *= quarter;
            rotate.x *= quarter;
            obj.rotate = rotate;
        }

        obj.cornerIndex = cornerIndex;

        cubies.push(addCubie(obj));
    });
}


renderState();

const ghostCtx = ghostEl.getContext('2d');

element.addEventListener('click', (e) => {
    if (hasDragged) {
        hasDragged = false;
        return;
    }

    const { offsetX, offsetY } = e;

    const [colorIndex] = ghostCtx.getImageData(offsetX, offsetY, 1, 1).data;

    if (colorIndex) {
        const cubie = cubies.find((cubie) => cubie.colorIndex === colorIndex);

        if (cubie.isEdge) {
            const edgeOrientation = state.edges[cubie.edgeIndex];
            state.edges[cubie.edgeIndex] = (edgeOrientation + 1) % 4;
            renderState();
        } else if (cubie.isCorner) {
            const toggledCubie = cubies.find((cubie) => cubie.toggled);

            if (toggledCubie) {
                toggledCubie.toggle();

                const [a, b] = [toggledCubie.cornerIndex, cubie.cornerIndex];

                const tmp = state.corners[a];
                state.corners[a] = state.corners[b];
                state.corners[b] = tmp;

                renderState();
            } else {
                cubie.toggle();
            }
        }
    }
});

function render() {
    requestAnimationFrame(render);
    illo.updateRenderGraph();
    ghost.updateRenderGraph();

    document.querySelector('pre').textContent = JSON.stringify(state, 0, 4);
}

render();
