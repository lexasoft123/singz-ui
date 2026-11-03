/** Geometry shared by native header slots. Widths are intrinsic action widths,
 * not fixed icon slots. Keep enough room to read a title at the current scale. */
export function headerLayout(width, leading, trailing, fontScale, align) {
    const available = Math.max(0, width - 22);
    const side = Math.max(leading, trailing);
    const identityRoom = available - (align === 'center' ? 2 * side : leading + trailing) - 16;
    const stackActions = width > 0 && trailing > 0 && identityRoom < 120 * Math.max(1, fontScale);
    return {
        stackActions,
        leadingSpace: align === 'center' && !stackActions ? Math.max(0, trailing - leading) : 0,
        trailingSpace: align === 'center' && !stackActions ? Math.max(0, leading - trailing) : 0
    };
}
