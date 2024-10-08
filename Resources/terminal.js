let html = `<h2>Send Shell Command</h2>
            <div>
                <label for="cmd">Command:</label>
                <input type="text" id="cmd" value="${platform=='win32'?'cmd':'ls'}">
                <label for="args">Arguments:</label>
                <input type="text" id="args" value="${platform=='win32'?'/C dir':'-ltr'}">
                <label for="stdin">stdin:</label>
                <input type="text" id="stdin">
                <button id="send">Send</button>
            </div>
            <div>
                <label for="cmd">Watch File Path:</label>
                <input type="text" class="path" id="path" value="/Users/thomastschurtschenthaler/Documents/workspaces/electrico">
                <button id="startwatch">Start</button>
                <button id="stopwatch">Stop</button>
            </div>
            <div>
                <label for="output">Output:</label>
                <div class="output" id="output"></div>
            </div>`;
document.getElementById("terminal").innerHTML=html;
document.getElementById("send").onclick = (e) => {
    let cmd = document.getElementById("cmd").value.trim();
    let args = document.getElementById("args").value.trim();
    let stdin = document.getElementById("stdin").value.trim();
    args = args=args!=""?args.split(" "):null;
    callshell({cmd:cmd, args:args, stdin:stdin});
};
document.getElementById("startwatch").onclick = (e) => {
    let path = document.getElementById("path").value.trim();
    startwatch(path);
};
document.getElementById("stopwatch").onclick = (e) => {
    stopwatch();
};
window.onWriteOutput((event, text, level) => {
    text = text.replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll("\r\n", "<br>").replaceAll("\n", "<br>").replaceAll("\r", "<br>");
    let html = level!=null?("<span class='"+level+"'>"+text+"</span>"):text;
    document.getElementById("output").innerHTML+=html;
});