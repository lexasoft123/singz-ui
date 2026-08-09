import { useEffect, type RefObject } from 'react'

/**
 * Close on a pointer press outside `ref`, or on Escape.
 *
 * Replaces three byte-identical copies in SingZ's Transport (the volume,
 * metronome and training popovers).
 *
 * Escape is handled in the CAPTURE phase and stops propagation, which is not
 * incidental: the app has a window-level key handler that clears the current
 * selection, and without this the same keypress would close the popover AND
 * wipe the selection underneath it.
 */
export function useDismissable(
  ref: RefObject<HTMLElement | null>,
  onClose: () => void,
  options: { enabled?: boolean; ignoreOutside?: boolean } = {}
): void {
  const { enabled = true, ignoreOutside = false } = options
  useEffect(() => {
    if (!enabled) return
    const onDown = (e: PointerEvent): void => {
      if (ignoreOutside) return
      // The anchor (the button that opened this) counts as inside: its own
      // click handler does the closing, and treating it as outside makes the
      // popover close and immediately reopen.
      const host = ref.current?.parentElement ?? ref.current
      if (!host?.contains(e.target as Node)) onClose()
    }
    const onKey = (e: KeyboardEvent): void => {
      if (e.code === 'Escape') {
        e.stopPropagation()
        onClose()
      }
    }
    document.addEventListener('pointerdown', onDown)
    window.addEventListener('keydown', onKey, true)
    return () => {
      document.removeEventListener('pointerdown', onDown)
      window.removeEventListener('keydown', onKey, true)
    }
  }, [ref, onClose, enabled, ignoreOutside])
}
