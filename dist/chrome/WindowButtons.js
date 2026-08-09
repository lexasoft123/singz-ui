import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import { useEffect, useState } from 'react';
const DEFAULT_LABELS = {
    minimize: 'Minimize',
    maximize: 'Maximize',
    restore: 'Restore',
    close: 'Close'
};
/**
 * Custom min/max/close for a frameless window (the native Windows 10 frame is
 * square, which is why the app draws its own).
 *
 * Exactly three buttons, close last — the smoke test asserts the count and
 * finds `.close` by position. Also mirrors the maximized state onto
 * <body class="maximized">, which the chrome CSS uses to flatten the window's
 * rounded corners when it fills the screen.
 */
export function WindowButtons({ api, labels = DEFAULT_LABELS }) {
    const [maximized, setMaximized] = useState(false);
    useEffect(() => {
        const apply = (v) => {
            setMaximized(v);
            document.body.classList.toggle('maximized', v);
        };
        void api.isMaximized().then(apply);
        return api.onMaximized(apply);
    }, [api]);
    return (_jsxs("div", { className: "win-controls no-drag", children: [_jsx("button", { type: "button", title: labels.minimize, onClick: () => api.minimize(), children: _jsx("svg", { width: "10", height: "10", viewBox: "0 0 10 10", "aria-hidden": true, children: _jsx("rect", { x: "0", y: "4.5", width: "10", height: "1", fill: "currentColor" }) }) }), _jsx("button", { type: "button", title: maximized ? labels.restore : labels.maximize, onClick: () => api.maximizeToggle(), children: maximized ? (_jsx("svg", { width: "10", height: "10", viewBox: "0 0 10 10", "aria-hidden": true, children: _jsx("path", { d: "M2.5 2.5V0.5h7v7h-2M0.5 2.5h7v7h-7z", fill: "none", stroke: "currentColor", strokeWidth: "1" }) })) : (_jsx("svg", { width: "10", height: "10", viewBox: "0 0 10 10", "aria-hidden": true, children: _jsx("rect", { x: "0.5", y: "0.5", width: "9", height: "9", fill: "none", stroke: "currentColor", strokeWidth: "1" }) })) }), _jsx("button", { type: "button", className: "close", title: labels.close, onClick: () => api.close(), children: _jsx("svg", { width: "10", height: "10", viewBox: "0 0 10 10", "aria-hidden": true, children: _jsx("path", { d: "M0.7 0.7l8.6 8.6M9.3 0.7L0.7 9.3", stroke: "currentColor", strokeWidth: "1.1" }) }) })] }));
}
/**
 * Put `mac` / `win` on <body> so the platform rules apply. Call once, before
 * the first render — the app reads these classes DURING render to decide
 * whether to mount the window buttons, so setting them later leaves the CSS
 * and the chrome disagreeing until something else re-renders.
 */
export function applyPlatformClasses(ua = navigator.userAgent) {
    document.body.classList.toggle('mac', ua.includes('Macintosh'));
    document.body.classList.toggle('win', ua.includes('Windows'));
}
