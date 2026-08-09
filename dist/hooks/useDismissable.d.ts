import { type RefObject } from 'react';
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
export declare function useDismissable(ref: RefObject<HTMLElement | null>, onClose: () => void, options?: {
    enabled?: boolean;
    ignoreOutside?: boolean;
}): void;
