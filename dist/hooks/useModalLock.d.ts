/**
 * Mark the app as covered by a modal, for as long as this component is
 * mounted, by putting `modal-open` on <body>.
 *
 * REF-COUNTED, which is the whole reason this exists. SingZ had two
 * independent owners of that class — App.tsx toggling it for its six modals,
 * and DropScreen toggling it for its own inline confirm — so closing either
 * one cleared the flag even while the other was still up. A counter cannot
 * get that wrong.
 *
 * The flag is not cosmetic. Background rAF loops read it and skip their DOM
 * writes while it is set, because every invalidated pixel under the scrim's
 * backdrop-filter forces a full-window recomposite.
 */
export declare function useModalLock(active?: boolean): void;
/** True while any modal covers the app. Cheap enough to call per frame. */
export declare function modalCoversApp(): boolean;
