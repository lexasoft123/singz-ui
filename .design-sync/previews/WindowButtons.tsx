import { WindowButtons, type WindowControlsApi } from '@singz/ui'

/*
 * Frameless-window chrome. The five main-process calls are INJECTED — that is
 * the whole reason this component is shareable, so the preview shows the
 * adapter rather than hiding it. A host wires these to its own IPC bridge.
 *
 * `.win-controls` is `position: fixed; top: 0; right: 0`, so each cell gives it
 * a positioned box to sit in instead of letting it escape to the viewport.
 */

const noopApi = (maximized: boolean): WindowControlsApi => ({
  isMaximized: () => Promise.resolve(maximized),
  onMaximized: () => () => {},
  minimize: () => {},
  maximizeToggle: () => {},
  close: () => {}
})

const Titlebar = ({
  label,
  maximized
}: {
  label: string
  maximized: boolean
}): React.JSX.Element => (
  <div style={{ marginBottom: 12 }}>
    <div style={{ fontSize: 11, color: 'var(--sz-dim)', marginBottom: 4 }}>{label}</div>
    <div
      style={{
        position: 'relative',
        height: 52,
        background: 'var(--sz-panel)',
        border: '1px solid var(--sz-line)',
        borderRadius: 8,
        overflow: 'hidden',
        /* .win-controls is position:fixed — only a transform (or filter /
           perspective) makes an ancestor its containing block. Without this
           both instances escape to the viewport corner and overlap. Hosts
           embedding the chrome anywhere but the real window need the same. */
        transform: 'translateZ(0)',
        display: 'flex',
        alignItems: 'center',
        paddingLeft: 14
      }}
    >
      <span style={{ fontWeight: 700, letterSpacing: '0.02em' }}>
        Sing<span style={{ color: 'var(--sz-accent)' }}>Z</span>
      </span>
      {/* fixed → absolute so the chrome stays inside this box */}
      <div style={{ position: 'absolute', inset: 0 }}>
        <div style={{ position: 'absolute', top: 0, right: 0 }}>
          <WindowButtons api={noopApi(maximized)} />
        </div>
      </div>
    </div>
  </div>
)

const Surface = ({ children }: { children: React.ReactNode }): React.JSX.Element => (
  <div
    style={{
      background: 'var(--sz-bg)',
      color: 'var(--sz-text)',
      fontFamily: 'var(--sz-font-display)',
      padding: 20,
      borderRadius: 10
    }}
  >
    {children}
  </div>
)

/**
 * Three buttons, close last. The middle one's tooltip flips Maximize ⇄ Restore
 * with the window state — the app's Windows smoke test selects on exactly that.
 */
export const Chrome = (): React.JSX.Element => (
  <Surface>
    <Titlebar label="Restored — middle button reads “Maximize”" maximized={false} />
    <Titlebar label="Maximized — it now reads “Restore”" maximized />
  </Surface>
)
