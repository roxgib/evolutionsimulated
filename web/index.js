const canvas = document.getElementById("world");
const viewer = document.getElementById("viewer");
let sim_speed = 90;

import init, { initialise, reinitialise, tick, get_config, render, get_world_data, on_click, render_selected, update_config } from './pkg/evolution_simulated.js';

run();

async function run() {
    await init();
    
    canvas.addEventListener('click', onClick, false);
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    viewer.width = 150;
    viewer.height = 150;

    initialise(window.innerWidth, window.innerHeight);
    initialise_inputs();
    renderLoop();
    _tick();
}

function _reinitialise() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    reinitialise(window.innerWidth, window.innerHeight);

    let config = JSON.parse(get_config());
    for (let key in config) {
        try {
            _update_config(key);
        } catch (err) {
            console.log(`Error: Couldn't update ${key} while resetting: ${err}`);
        }
    }
}

function renderLoop() {
    render(canvas, canvas.getContext("2d"));
    if (document.getElementById("infobar").classList.contains("shown")) {
        render_selected(viewer, viewer.getContext("2d"));
    }
    requestAnimationFrame(renderLoop);
}

function _tick() {
    tick();
    update_world_data();
    setTimeout(_tick, sim_speed);
}

function onClick(event) {
    let x = event.pageX;
    let y = event.pageY;
    let data = on_click(x, y)
    if (data != null) {
        document.getElementById("infobar").classList.add("shown")
        data = data.split("###");
        document.getElementById("eye_genes").textContent = data[0]
        document.getElementById("skin_genes").textContent = data[1];
        document.getElementById("speed_genes").textContent = data[2];
    } else {
        document.getElementById("infobar").classList.remove("shown")
    }
}

function _update_config(key) {
    try {
        let input = document.getElementById(key);
        if (key == "food_blocks_repr" || key == "movers_can_leaf") {
            update_config(input.name, `${input.checked}`);
        } else {
            update_config(input.name, input.value);
        }
    } catch (err) {
        console.log(`Error: Couldn't get input for ${key}: ${err}`);
    }
}

function initialise_inputs() {
    let config = JSON.parse(get_config());
    for (let key in config) {
        let input;
        try {
            input = document.getElementById(key);
            input.value = config[key];
            input.checked = config[key];
            input.defaultValue = config[key];
            input.defaultChecked = config[key];
            input.addEventListener("change", () => _update_config(input.id));
        } catch (err) {
            console.log(`Error: Couldn't get input for ${key}: ${err}`);
        }
    }
    document.getElementById("reset_sim").addEventListener("click", _reinitialise);
    document.getElementById("sim_speed").addEventListener("change", () => {
        sim_speed = 150 - document.getElementById("sim_speed").value;
    });
}

function update_world_data() {
    let info = JSON.parse(get_world_data());
    for (let key in info) {
        document.getElementById(key.toLowerCase()).textContent = add_separators(
            `${info[key]}`
        );
    }
    if (info["Population"] == "0") {
        _reinitialise();
    }
}

function add_separators(s) {
    let length = s.length;
    while (length > 3) {
        s = s.slice(0, length - 3) + " " + s.slice(length - 3);
        length -= 3;
    }
    return s;
}
