/*
 * @singz/ui — the night-studio design language.
 *
 * The kit takes ownership of the design one layer at a time, and each step
 * has to prove it changed nothing: moving WHERE a value comes from and
 * changing WHAT it is must never land in the same commit, or nothing can say
 * which of the two moved a pixel.
 *
 *   v0.1.0  tokens
 *   v0.2.0  primitives  <- here
 */
export { tokens, atelier, cssVar, toCss } from './tokens/tokens.js';
export { cx } from './util/cx.js';
export { Button } from './primitives/Button.js';
export { Chip } from './primitives/Chip.js';
export { SegmentedControl } from './primitives/SegmentedControl.js';
export { StatusDot, LinkButton, Badge } from './primitives/StatusDot.js';
export { Modal, ModalActions } from './overlays/Modal.js';
export { useDismissable } from './hooks/useDismissable.js';
export { useModalLock, modalCoversApp } from './hooks/useModalLock.js';
export { Waveform } from './audio/Waveform.js';
export { fitCanvas } from './hooks/useCanvas2D.js';
export { WindowButtons, applyPlatformClasses } from './chrome/WindowButtons.js';
export { STEM_META, CUSTOM_COLORS, STEM_ORDER } from './tokens/stems.js';
