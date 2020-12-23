export type Shape = SimpleShape | ComplexShape;
export enum SimpleShape{
    Square = "Square",
    Circle = "Circle",
}
export type ComplexShape = { Plus: Ratio } | { VerticalBar: Ratio } | { HorizontalBar: Ratio };
export class Ratio {
    ratio: number;
    
    constructor(ratio: number) {
        this.ratio = ratio;
    }
}
