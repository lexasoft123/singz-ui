/** Join class names, dropping anything falsy. Replaces 28 hand-rolled
 *  template literals of the shape `chip mute${x ? ' active' : ''}`. */
export declare function cx(...parts: (string | false | null | undefined)[]): string;
