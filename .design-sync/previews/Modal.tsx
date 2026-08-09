import { Button, Modal, ModalActions } from '@singz/ui'

/*
 * Modal renders a fixed, full-viewport scrim, so each cell is captured on its
 * own (cfg.overrides.Modal cardMode "single") — otherwise the scrim escapes
 * the grid cell. Compositions are SingZ's real dialogs.
 */

export const Confirm = (): React.JSX.Element => (
  <Modal onClose={() => {}} cardClassName="confirm-card" aria-label="Delete project">
    <h2>Delete “Sixteen Tons”?</h2>
    <p>
      This erases the whole project folder — 6 stems, its lyrics, your mix, transpose and
      training settings, 47 MB in all. It does not go to the Trash and it cannot be undone here.
    </p>
    <ModalActions>
      <Button variant="ghost" size="sm">
        Keep it
      </Button>
      <Button variant="ghost" size="sm" className="danger">
        Delete 47 MB
      </Button>
    </ModalActions>
  </Modal>
)

/** A dialog that must be answered rather than dismissed: no Escape, no scrim click. */
export const Persistent = (): React.JSX.Element => (
  <Modal onClose={() => {}} persistent cardClassName="wizard" aria-label="AI models">
    <h2>AI models</h2>
    <p>
      SingZ runs its AI locally. Models download once into a shared folder and are reused for
      every song.
    </p>
    <ModalActions>
      <Button variant="primary">Get · 1200 MB</Button>
      <Button variant="ghost">Not now</Button>
    </ModalActions>
  </Modal>
)

/**
 * `busy` blocks dismissal — Escape and the scrim both stop working. It is
 * purely behavioural: it paints nothing, and a disabled `ghost` looks live, so
 * the host dims the actions itself to say “in flight”.
 */
export const Busy = (): React.JSX.Element => (
  <Modal onClose={() => {}} busy cardClassName="confirm-card" aria-label="Deleting">
    <h2>Deleting “Wanted Dead Or Alive”…</h2>
    <p className="fine">The copy in Google Drive moves to Drive’s trash on the next sync.</p>
    <ModalActions>
      <Button variant="ghost" size="sm" disabled style={{ opacity: 0.45, cursor: 'default' }}>
        Keep it
      </Button>
      <Button
        variant="ghost"
        size="sm"
        className="danger"
        disabled
        style={{ opacity: 0.45, cursor: 'default' }}
      >
        Deleting…
      </Button>
    </ModalActions>
  </Modal>
)
