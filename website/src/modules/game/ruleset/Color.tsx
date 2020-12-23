export class Color{
    color: string;
    
    constructor(r: number, g: number, b: number, a: number) {
        function clamp(val: number): number{
            return Math.max(Math.min(Math.floor(val), 255), 0);
        }
        
        this.color = "rgba(" + clamp(r) + ", " + clamp(g) + ", " + clamp(b) + ", " + clamp(a) + ")";
    }
}
