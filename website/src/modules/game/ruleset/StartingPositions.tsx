import {Coordinate} from "../Coordinate";

export type StartingPositions =
    { MirroredFlipped: PositionsForSeat }
    | { MirroredRotated: PositionsForSeat }
    | { NotMirrored: PositionsForSeat[] }
    | { Placement: Placement};

export type PositionsForSeat = Coordinate[][];
export class Placement{
    first_seat: number;
    alteration_type: AlterationType;
    placement_area: PlacementArea;
    piece_limits: PieceLimit[];


    constructor(first_seat: number, alteration_type: AlterationType, placement_area: PlacementArea, piece_limits: PieceLimit[]) {
        this.first_seat = first_seat;
        this.alteration_type = alteration_type;
        this.placement_area = placement_area;
        this.piece_limits = piece_limits;
    }
}

export type AlterationType =
    AlterationTypeSimple
    | { TurnsCount: { per_turn_count: number }}
    | { TurnsPoints: { per_turn_points: number, hard_limit: boolean }};
export enum AlterationTypeSimple{
    Points = "Points",
    WholePlacement = "WholePlacement",
    Hidden = "Hidden",
}

export type PlacementArea =
    PlacementAreaSimple
    | { MirroredFlipped: Coordinate[] }
    | { MirroredRotated: Coordinate[] }
    | { NonMirrored: Coordinate[][] };
export enum PlacementAreaSimple{
    Half = "Half",
}

export type PieceLimit =
    { TotalLimit: { limit: number } }
    | { TypeCountLimit: { limits: number[] } }
    | { PointLimit: { point_values: number[], point_limit: number } };
