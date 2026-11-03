/** Geometry shared by native header slots. Widths are intrinsic action widths,
 * not fixed icon slots. Keep enough room to read a title at the current scale. */
export declare function headerLayout(width: number, leading: number, trailing: number, fontScale: number, align: 'center' | 'start'): {
    readonly stackActions: boolean;
    readonly leadingSpace: number;
    readonly trailingSpace: number;
};
