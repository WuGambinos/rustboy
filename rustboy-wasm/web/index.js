import init, {boot, WebGameBoy}  from './rustboy_wasm.js';

// start wasm
async function start_wasm() {
    await init();

    // Load file button
    var button = document.getElementById('btn');
    button.addEventListener('click', () => {
        loadFile();
    })

    // Animation loop
    //
    /*
    function loop() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        render_buffer();
        requestAnimationFrame(loop);
    }
    */
    /*
    function loop(gb) {
        requestAnimationFrame(loop);
    }
    */


    async function loadFile() {
        const inputElement = document.getElementById('fileInput');
        const file = inputElement.files[0];

        console.log("ATTEMPTING TO LOAD FILE");
        if (!file) {
            alert('Please select a file.');
            return;
        }
        console.log("LOADING FILE");
        const fileContents = await readFileAsArrayBuffer(file);
        const contents = new Uint8Array(fileContents);

        boot(contents);
        let gb = new WebGameBoy();
        gb.boot(contents);
        game_loop();

        function game_loop() {
            gb.run();
            gb.draw();
            requestAnimationFrame(game_loop);
        }
    }

    const readFileAsArrayBuffer = (file) => {
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
start_wasm();
