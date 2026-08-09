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
export { tokens, cssVar, toCss, type TokenName } from './tokens/tokens.js'
export { cx } from './util/cx.js'
export { Button, type ButtonProps, type ButtonVariant } from './primitives/Button.js'
export { Chip, type ChipProps } from './primitives/Chip.js'
export {
  SegmentedControl,
  type SegmentedControlProps,
  type SegmentOption
} from './primitives/SegmentedControl.js'
export {
  StatusDot,
  LinkButton,
  Badge,
  type DotTone,
  type StatusDotProps,
  type LinkButtonProps,
  type BadgeProps
} from './primitives/StatusDot.js'
export { Modal, ModalActions, type ModalProps, type ModalActionsProps } from './overlays/Modal.js'
export { useDismissable } from './hooks/useDismissable.js'
export { useModalLock, modalCoversApp } from './hooks/useModalLock.js'
export { Waveform, type WaveformProps } from './audio/Waveform.js'
export { fitCanvas } from './hooks/useCanvas2D.js'
export {
  WindowButtons,
  applyPlatformClasses,
  type WindowButtonsProps,
  type WindowControlsApi
} from './chrome/WindowButtons.js'
export { STEM_META, CUSTOM_COLORS, STEM_ORDER, type StemMeta } from './tokens/stems.js'
