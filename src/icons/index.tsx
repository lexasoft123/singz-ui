import { createElement, type CSSProperties, type JSX } from 'react'
import { iconArtwork, type IconName } from './artwork.js'
export type { IconName } from './artwork.js'
export interface IconProps { name: IconName; size?: number; color?: string; label?: string; className?: string; style?: CSSProperties }
/** Decorative unless a label is supplied. Icon-only buttons still need their own accessible name. */
export function Icon({ name, size = 24, color = 'currentColor', label, className, style }: IconProps): JSX.Element {
 return createElement('svg', { width: size, height: size, viewBox: '0 0 28 28', fill: 'none', stroke: color, color, strokeWidth: 2.5, strokeLinecap: 'round', strokeLinejoin: 'round', role: label ? 'img' : undefined, 'aria-label': label, 'aria-hidden': label ? undefined : true, focusable: 'false', className, style: { flexShrink: 0, verticalAlign: 'middle', ...style }, dangerouslySetInnerHTML: { __html: iconArtwork[name] } })
}
