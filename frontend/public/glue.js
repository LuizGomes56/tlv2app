const invoke = window.__TAURI_INTERNALS__?.invoke;

export async function invoke_send_code() {
    return await invoke?.("send_code");
}
export async function invoke_get_realtime_game() {
    return await invoke?.("get_realtime_game");
}

export async function invoke_calculate(gameState) {
    return await invoke?.("get_calculator_value", { gameState });
}