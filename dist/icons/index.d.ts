import { type CSSProperties, type JSX } from 'react';
import { type IconName } from './artwork.js';
export type { IconName } from './artwork.js';
export interface IconProps {
    name: IconName;
    size?: number;
    color?: string;
    label?: string;
    className?: string;
    style?: CSSProperties;
}
/** Decorative unless a label is supplied. Icon-only buttons still need their own accessible name. */
export declare function Icon({ name, size, color, label, className, style }: IconProps): JSX.Element;
