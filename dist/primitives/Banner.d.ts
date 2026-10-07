export interface BannerProps extends React.HTMLAttributes<HTMLDivElement> {
    tone?: 'info' | 'danger';
    icon?: React.ReactNode;
    heading?: React.ReactNode;
    children?: React.ReactNode;
    footer?: React.ReactNode;
}
/** Centered status or feedback, with a full-width optional second line. */
export declare function Banner({ tone, icon, heading, children, footer, className, ...rest }: BannerProps): React.JSX.Element;
