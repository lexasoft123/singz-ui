/** Join class names, dropping anything falsy. Replaces 28 hand-rolled
 *  template literals of the shape `chip mute${x ? ' active' : ''}`. */
export function cx(...parts: (string | false | null | undefined)[]): string {
  return parts.filter(Boolean).join(' ')
}
