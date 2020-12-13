const input = document.getElementById("input") as HTMLTextAreaElement;
const submit_button = document.getElementById("submit_button") as HTMLButtonElement;
const result = document.getElementById("result") as HTMLDivElement;

const web_socket = new WebSocket("ws://" + location.host + "/echo");
web_socket.onmessage = function (event) {
    let text = document.createElement("P")
    text.textContent = event.data;
    result.appendChild(text);
}

submit_button.onclick = function (_) {
    console.log("Value: " + input.value);
    web_socket.send(input.value);
}
