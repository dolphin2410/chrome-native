let communication = document.getElementById("communication");

communication?.addEventListener("click", async () => {
  chrome.runtime.sendNativeMessage('me.dolphin2410.chrome_native',
  { task: "FileCreateTask", message: { file_name: "hello.txt", content: "DONE!!" } },
  (response) => {
    console.log("Received " + JSON.stringify(response));
  })
});