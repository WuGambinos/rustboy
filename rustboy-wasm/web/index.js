import init,  {WebGameBoy}  from './rustboy_wasm.js';

// FPS COUNTER
const fps = new class {
    constructor()  {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now()
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        this.frames.push(fps);
        if (this.frames.length > 100){
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;

        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min  = Math.min(this.frames[i], min);
            max  = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

           // Render the statistics.
        this.fps.textContent = ` Frames per Second:
            latest = ${Math.round(fps)}
            avg of last 100 = ${Math.round(mean)}
            min of last 100 = ${Math.round(min)}
            max of last 100 = ${Math.round(max)}`.trim();
    }
}

async function startWasm() {
    await init();

    let load_button = document.getElementById('load');
    load_button.addEventListener('click', () => {
        loadFile();
    });

    let resetPressed = false;
    let resetButton = document.getElementById('reset');
    resetButton.addEventListener('click', () => {
        resetPressed = true;
    });

    // Handle Keys
    let keyPressed = null;
    let keyReleased = null;
    window.addEventListener('keydown', function (e) {
        if (e.key === "Space" || e.key === "ArrowDown"
        || e.key === "ArrowUp" || e.key === "ArrowLeft"
        || e.key === "ArrowRight"){
            e.preventDefault();
        }
        console.log("YOU PRESED: " + e.key);
        keyPressed = e.key;
    }, false);

    window.addEventListener("keyup", (e) => {
        if (e.isComposing || e.keyCode === 229) {
            return;
        }
        if (e.key === "Space" || e.key === "ArrowDown"
        || e.key === "ArrowUp" || e.key === "ArrowLeft"
        || e.key === "ArrowRight") {
            e.preventDefault();
        }
        keyReleased = e.key;
    });

    async function loadFile() {
        const inputElement = document.getElementById('fileInput');
        const file = inputElement.files[0];

        if (!file) {
            alert('Please select a file.');
            return;
        }
        console.log("LOADING FILE");
        const fileContents = await readFileAsArrayBuffer(file);
        const contents = new Uint8Array(fileContents);

        let canvas = document.getElementById('canvas');
        let context = canvas.getContext('2d');

        let gb = new WebGameBoy();
        gb.boot(contents);
        game_loop();

        function game_loop() {
            fps.render();
            gb.run();
            gb.draw();
            if (keyPressed != null) {
                gb.on_key_down(keyPressed);
                keyPressed = null;
            }

            if (keyReleased != null) {
                gb.on_key_up(keyReleased);
                keyReleased = null;
            }

            if (resetPressed) {
                gb.reset();
                gb.boot(contents);
                resetPressed = false;
                context.clearRect(0, 0, canvas.width, canvas.height);

            }

            requestAnimationFrame(game_loop);
        }
    }

    function readFileAsArrayBuffer(file) {
        return new Promise((resolve, reject) => {
        const reader = new FileReader();
            reader.onload = (event) => {
              resolve(event.target.result);
            };
            reader.onerror = (error) => {
          reject(error);
        };
        reader.readAsArrayBuffer(file);
      });
    };
}
startWasm();
