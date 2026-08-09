import { type ButtonHTMLAttributes, type ReactNode } from 'react';
export interface ChipProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    /** Pressed state. Also sets aria-pressed — SingZ had this on 5 of its
     *  toggles and missing on the rest; centralising it fixes that by
     *  construction rather than by remembering. */
    active?: boolean;
    className?: string;
    children?: ReactNode;
}
/** A small square toggle — the M / S buttons on a mixer lane, and friends. */
export declare const Chip: import("react").ForwardRefExoticComponent<ChipProps & import("react").RefAttributes<HTMLButtonElement>>;
