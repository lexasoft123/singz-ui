/** Join class names, dropping anything falsy. Replaces 28 hand-rolled
 *  template literals of the shape `chip mute${x ? ' active' : ''}`. */
export function cx(...parts) {
    return parts.filter(Boolean).join(' ');
}
