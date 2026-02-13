/**
 * Creates an input handler for keyboard input.
 * @param {Object} keyMappings - Map of action names to arrays of key codes
 * @returns {Object} Object with action properties that track boolean state
 */
export function createInputHandler(keyMappings) {
    const keys = {};

    // Initialize all actions to false
    for (const action of Object.keys(keyMappings)) {
        keys[action] = false;
    }

    window.addEventListener('keydown', (e) => {
        for (const [action, keyCodes] of Object.entries(keyMappings)) {
            if (keyCodes.includes(e.key)) {
                keys[action] = true;
                e.preventDefault();
            }
        }
    });

    window.addEventListener('keyup', (e) => {
        for (const [action, keyCodes] of Object.entries(keyMappings)) {
            if (keyCodes.includes(e.key)) {
                keys[action] = false;
            }
        }
    });

    return keys;
}
