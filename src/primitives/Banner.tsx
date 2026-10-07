import { cx } from '../util/cx.js'

export interface BannerProps extends React.HTMLAttributes<HTMLDivElement> {
  tone?: 'info' | 'danger'
  icon?: React.ReactNode
  heading?: React.ReactNode
  children?: React.ReactNode
  footer?: React.ReactNode
}

/** Centered status or feedback, with a full-width optional second line. */
export function Banner({ tone = 'info', icon, heading, children, footer, className, ...rest }: BannerProps): React.JSX.Element {
  return (
    <div role={tone === 'danger' ? 'alert' : 'status'} {...rest} className={cx('sz-banner', `sz-banner--${tone}`, className)}>
      <div className="sz-banner__content">
        {icon && <span className="sz-banner__icon" aria-hidden="true">{icon}</span>}
        {heading && <strong className="sz-banner__heading">{heading}</strong>}
        {children && <span className="sz-banner__detail">{children}</span>}
      </div>
      {footer && <div className="sz-banner__footer">{footer}</div>}
    </div>
  )
}
