/*
 * @singz/ui — the night-studio design language.
 *
 * v0.1.0 is deliberately tokens-only. The kit takes ownership of the design
 * in one dimension at a time, and each step has to prove it changed nothing:
 * moving WHERE a value comes from and changing WHAT it is must never land in
 * the same commit, or you lose the ability to say which one moved a pixel.
 */
export { tokens, cssVar, toCss, type TokenName } from './tokens/tokens.js'
