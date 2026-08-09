import { Button, Modal, ModalActions } from '@singz/ui'

/*
 * ModalActions is the footer row of a dialog. It is shown inside a real Modal
 * rather than alone: that is the only place it appears, and the app's own
 * end-to-end tests select it as `.settings-card .modal-actions .pill`, so the
 * nesting IS the contract.
 */

export const TwoActions = (): React.JSX.Element => (
  <Modal onClose={() => {}} aria-label="Stem splitting">
    <h2>Stem splitting needs Demucs</h2>
    <p className="fine">Install it once and every future split uses it.</p>
    <ModalActions>
      <Button variant="primary">Re-check</Button>
      <Button variant="ghost">Close</Button>
    </ModalActions>
  </Modal>
)

/**
 * The row wraps rather than overflowing once the actions outgrow the card —
 * shown here at the card's natural width, so this is the real wrap point.
 * Card width itself comes from `cardClassName`, which the HOST owns
 * (`.settings-card`, `.picker-card`); the kit ships no sizing.
 */
export const ManyActions = (): React.JSX.Element => (
  <Modal onClose={() => {}} cardClassName="settings-card" aria-label="Library location">
    <h2>Where your projects live</h2>
    <p className="fine">Cloud folders are just folders — point SingZ at one and the phones follow.</p>
    <ModalActions>
      <Button variant="ghost" size="sm">
        Use iCloud Drive
      </Button>
      <Button variant="ghost" size="sm">
        Use Dropbox
      </Button>
      <Button variant="ghost" size="sm">
        Choose folder…
      </Button>
      <Button variant="ghost" size="sm">
        Back to Documents
      </Button>
      <Button variant="ghost" size="sm">
        Close
      </Button>
    </ModalActions>
  </Modal>
)
