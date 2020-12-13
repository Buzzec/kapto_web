const chat = document.getElementById("chat") as HTMLDivElement;
const text = document.getElementById("text") as HTMLInputElement;
const send = document.getElementById("send") as HTMLButtonElement;
const uri = "ws://" + location.host + "/chat?username=" + new URLSearchParams(window.location.search).get("username");
const ws = new WebSocket(uri);

function present_message(data: string) {
    const line = document.createElement("p");
    line.innerText = data;
    chat.appendChild(line);
}

ws.onopen = function () {
    chat.innerHTML = "<p><em>Connected!</em></p>"
}
ws.onmessage = function (msg) {
    present_message(msg.data);
}
ws.onclose = function () {
    chat.getElementsByTagName("em")[0].innerText = "Disconnected!";
}

send.onclick = function () {
    const message = text.value;
    ws.send(message);
    text.value = "";
    present_message("<You>: " + message)
}
