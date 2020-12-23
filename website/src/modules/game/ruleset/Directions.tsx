export class Directions{
    bits: number;

    constructor(literal: DirectionLiteral) {
        this.bits = literal;
    }
    
    or(other: Directions): Directions{
        let out: Directions;
        Object.assign(out, this);
        out.bits = out.bits | other.bits;
        return out;
    }
}

export enum DirectionLiteral{
    NORTH         = 0b00000001,
    SOUTH         = 0b00000010,
    EAST          = 0b00000100,
    WEST          = 0b00001000,
    NORTH_WEST    = 0b00010000,
    NORTH_EAST    = 0b00100000,
    SOUTH_WEST    = 0b01000000,
    SOUTH_EAST    = 0b10000000,
    CARDINAL      = NORTH | SOUTH | EAST | WEST,
    DIAGONAL      = NORTH_WEST | NORTH_EAST | SOUTH_WEST | SOUTH_EAST,
    ALL           = CARDINAL | DIAGONAL,
    NONE          = 0b00000000,
}
