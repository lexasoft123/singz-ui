import { useEffect, useState } from 'react'

/**
 * The four main-process calls this needs. Injected rather than imported so
 * the kit never reaches for a particular app's IPC bridge — SingZ exposes
 * `window.singz`, the next app will call it something else.
 */
export interface WindowControlsApi {
  isMaximized: () => Promise<boolean>
  /** Subscribe; returns an unsubscribe function. */
  onMaximized: (cb: (v: boolean) => void) => () => void
  minimize: () => void
  maximizeToggle: () => void
  close: () => void
}

export interface WindowButtonsProps {
  api: WindowControlsApi
  /**
   * Button tooltips. These are NOT decoration: the Windows smoke test
   * selects `.win-controls button[title="Maximize"]` and waits for the title
   * to flip to "Restore", so the defaults are a contract. Override only if
   * you are also localising the tests.
   */
  labels?: { minimize: string; maximize: string; restore: string; close: string }
}

const DEFAULT_LABELS = {
  minimize: 'Minimize',
  maximize: 'Maximize',
  restore: 'Restore',
  close: 'Close'
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
export function WindowButtons({ api, labels = DEFAULT_LABELS }: WindowButtonsProps): React.JSX.Element {
  const [maximized, setMaximized] = useState(false)

  useEffect(() => {
    const apply = (v: boolean): void => {
      setMaximized(v)
      document.body.classList.toggle('maximized', v)
    }
    void api.isMaximized().then(apply)
    return api.onMaximized(apply)
  }, [api])

  return (
    <div className="win-controls no-drag">
      <button type="button" title={labels.minimize} onClick={() => api.minimize()}>
        <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden>
          <rect x="0" y="4.5" width="10" height="1" fill="currentColor" />
        </svg>
      </button>
      <button
        type="button"
        title={maximized ? labels.restore : labels.maximize}
        onClick={() => api.maximizeToggle()}
      >
        {maximized ? (
          <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden>
            <path
              d="M2.5 2.5V0.5h7v7h-2M0.5 2.5h7v7h-7z"
              fill="none"
              stroke="currentColor"
              strokeWidth="1"
            />
          </svg>
        ) : (
          <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden>
            <rect
              x="0.5"
              y="0.5"
              width="9"
              height="9"
              fill="none"
              stroke="currentColor"
              strokeWidth="1"
            />
          </svg>
        )}
      </button>
      <button type="button" className="close" title={labels.close} onClick={() => api.close()}>
        <svg width="10" height="10" viewBox="0 0 10 10" aria-hidden>
          <path d="M0.7 0.7l8.6 8.6M9.3 0.7L0.7 9.3" stroke="currentColor" strokeWidth="1.1" />
        </svg>
      </button>
    </div>
  )
}

/**
 * Put `mac` / `win` on <body> so the platform rules apply. Call once, before
 * the first render — the app reads these classes DURING render to decide
 * whether to mount the window buttons, so setting them later leaves the CSS
 * and the chrome disagreeing until something else re-renders.
 */
export function applyPlatformClasses(ua: string = navigator.userAgent): void {
  document.body.classList.toggle('mac', ua.includes('Macintosh'))
  document.body.classList.toggle('win', ua.includes('Windows'))
}
