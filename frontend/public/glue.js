const invoke = window.__TAURI_INTERNALS__?.invoke;

export async function invokeSendCode() {
    return await invoke?.("send_code");
}

/**
 * 
 * @param {string} gameCode
 * @returns {void}
 */
export async function invokeGetRealtimeGame(gameCode) {
    return await invoke?.("get_realtime_game", { gameCode });
}