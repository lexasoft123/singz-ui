export type DotTone = 'ok' | 'idle' | 'warn';
export interface StatusDotProps {
    tone: DotTone;
    className?: string;
}
/** A 7px state dot. `warn` pulses — and the host must pause that animation
 *  while a modal covers the app, or the scrim's backdrop blur re-rasters the
 *  whole window on every frame. SingZ does this via `body.modal-open`. */
export declare function StatusDot({ tone, className }: StatusDotProps): React.JSX.Element;
export interface LinkButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    className?: string;
}
/** `.linkish` — a text button that reads as a link but is a real button. */
export declare function LinkButton({ className, type, ...rest }: LinkButtonProps): React.JSX.Element;
export interface BadgeProps {
    children?: React.ReactNode;
    className?: string;
}
/** `.badge` — a small uppercase outline tag. */
export declare function Badge({ children, className }: BadgeProps): React.JSX.Element;
