import init,  {WebGameBoy}  from './rustboy_wasm.js';

// start wasm
async function start_wasm() {
    await init();

    // Load file button
    var button = document.getElementById('btn');
    button.addEventListener('click', () => {
        loadFile();
    })

    let keyPressed = null;
    let keyReleased = null;

    window.addEventListener('keydown', function (e) {
        keyPressed = e.key;
    }, false);

    window.addEventListener("keyup", (e) => {
          if (e.isComposing || e.keyCode === 229) {
            return;
          }
        keyReleased = e.key;
    });


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

        let gb = new WebGameBoy();
        gb.boot(contents);
        game_loop();

        function game_loop() {
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
