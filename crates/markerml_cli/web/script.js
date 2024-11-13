const webSocket = new WebSocket("ws://localhost:3002/listen");
const main = document.querySelector("#main");

const emptyContent = "<h1 class='full-screen empty-content'>No Code</h1>";
const errorContent = "<h1 class='full-screen error-content'>Compilation Error</h1>";

main.innerHTML = emptyContent;

webSocket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log(data);

    const { code, error } = data;
    if (error !== undefined) {
        main.innerHTML = errorContent;
    } else if (code !== undefined) {
        main.innerHTML = code;
    } else {
        main.innerHTML = emptyContent;
    }
};
