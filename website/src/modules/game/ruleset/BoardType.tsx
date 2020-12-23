export type BoardType = { Rectangular: BoardTypeRectangular } | { Custom: BoardTypeCustom };
export class BoardTypeRectangular{
    rows: number;
    columns: number;
    goal_locations: number[];
    
    constructor(rows: number, columns: number, goal_locations: number[]) {
        this.rows = rows;
        this.columns = columns;
        this.goal_locations = goal_locations;
    }
}
export class BoardTypeCustom{
    v: number;
    dim: number[];
    data: Space[];

    constructor(rows: number, columns: number) {
        this.v = 1;
        this.dim = [rows, columns];
        this.data = new Array(rows * columns);
        for(let value of this.data){
            value = SpaceSimple.Invalid;
        }
    }

    get_space(row: number, column: number): Space{
        if(row >= this.dim[0] || row < 0 || column >= this.dim[1] || column < 0){
            throw "Bad index in custom board";
        }
        return this.data[row * this.dim[1] + column];
    }
    set_space(row: number, column: number, space: Space){
        if(row >= this.dim[0] || row < 0 || column >= this.dim[1] || column < 0){
            throw "Bad index in custom board";
        }
        this.data[row * this.dim[1] + column] = space;
    }
}

export type Space = SpaceSimple | { Goal: number };
export enum SpaceSimple{
    Invalid = "Invalid",
    Normal = "Normal",
}

export function get_space(board: BoardType, row: number, column: number): Space{
    if("Rectangular" in board){
        const rectangle = board.Rectangular;
        if(row >= rectangle.rows + 2 || row < 0 || column >= rectangle.columns || column < 0){
            throw "Bad index in rectangular board";
        }
        if(row == 0 || row == rectangle.rows + 1){
            if(rectangle.goal_locations.includes(column)){
                // seat 0 is on top going down so row 0 is seat 1's goal
                return { Goal: row == 0 ? 1 : 0 };
            }
            else{
                throw "Column is not goal";
            }
        }
        return SpaceSimple.Normal;
    }
    else if("Custom" in board){
        return board.Custom.get_space(row, column);
    }
    else{
        throw "Board type not found!";
    }
}
