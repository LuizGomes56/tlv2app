const invoke = window.__TAURI_INTERNALS__?.invoke;

/**
 * 
 * @returns {string | undefined}
 */
export async function invokeGetGameCode() {
    return await invoke?.("get_game_code");
}

/**
 * 
 * @param {string} gameCode
 * @returns {void}
 */
export async function invokeGetRealtimeGame(gameCode) {
    return await invoke?.("get_realtime_game", { gameCode });
}

/**
 * 
 * @param {string} code 
 * @param {string} language 
 * @returns {string}
 */
export function highlightCode(code, language) {
    return Prism.highlight(code, Prism.languages[language]);
}