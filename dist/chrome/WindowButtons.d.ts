/**
 * The four main-process calls this needs. Injected rather than imported so
 * the kit never reaches for a particular app's IPC bridge — SingZ exposes
 * `window.singz`, the next app will call it something else.
 */
export interface WindowControlsApi {
    isMaximized: () => Promise<boolean>;
    /** Subscribe; returns an unsubscribe function. */
    onMaximized: (cb: (v: boolean) => void) => () => void;
    minimize: () => void;
    maximizeToggle: () => void;
    close: () => void;
}
export interface WindowButtonsProps {
    api: WindowControlsApi;
    /**
     * Button tooltips. These are NOT decoration: the Windows smoke test
     * selects `.win-controls button[title="Maximize"]` and waits for the title
     * to flip to "Restore", so the defaults are a contract. Override only if
     * you are also localising the tests.
     */
    labels?: {
        minimize: string;
        maximize: string;
        restore: string;
        close: string;
    };
}
/**
 * Custom min/max/close for a frameless window (the native Windows 10 frame is
 * square, which is why the app draws its own).
 *
 * Exactly three buttons, close last — the smoke test asserts the count and
 * finds `.close` by position. Also mirrors the maximized state onto
 * <body class="maximized">, which the chrome CSS uses to flatten the window's
 * rounded corners when it fills the screen.
 */
export declare function WindowButtons({ api, labels }: WindowButtonsProps): React.JSX.Element;
/**
 * Put `mac` / `win` on <body> so the platform rules apply. Call once, before
 * the first render — the app reads these classes DURING render to decide
 * whether to mount the window buttons, so setting them later leaves the CSS
 * and the chrome disagreeing until something else re-renders.
 */
export declare function applyPlatformClasses(ua?: string): void;
