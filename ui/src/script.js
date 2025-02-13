document.getElementById('ping').addEventListener('click', async () => {
    const response = await window.api.sendToRust("ping");
    document.getElementById('response').innerText = response.message;
});
