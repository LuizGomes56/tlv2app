const invoke = window.__TAURI_INTERNALS__.invoke;

export async function invokeSendCode() {
    return await invoke("send_code");
}
export async function invokeStartGame() {
    return await invoke("start_game");
}