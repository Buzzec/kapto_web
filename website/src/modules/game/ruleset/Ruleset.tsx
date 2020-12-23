import {Color} from "./Color";
import {PieceDefinition} from "./PieceDefinition";
import {BoardType, BoardTypeRectangular} from "./BoardType";
import {AlterationTypeSimple, Placement, PlacementAreaSimple, StartingPositions} from "./StartingPositions";
import {VictoryCondition, VictoryConditionsSimple} from "./VictoryCondition";

export class Ruleset{
    name: string;
    seats: number;
    allies: number[][];
    seat_colors: Color[];
    pieces: PieceDefinition[];
    board_type: BoardType;
    starting_positions: StartingPositions;
    victory_conditions: VictoryCondition;

    constructor(name: string, seats: number, allies: number[][], seat_colors: Color[], pieces: PieceDefinition[], board_type: BoardType, starting_positions: StartingPositions, victory_conditions: VictoryCondition) {
        this.name = name;
        this.seats = seats;
        this.allies = allies;
        this.seat_colors = seat_colors;
        this.pieces = pieces;
        this.board_type = board_type;
        this.starting_positions = starting_positions;
        this.victory_conditions = victory_conditions;
    }
    static default(): Ruleset{
        return new Ruleset(
            "Default Ruleset",
            2,
            [[]],
            [new Color(255, 0, 0, 255), new Color(0, 0, 255, 255)],
            [PieceDefinition.default()],
            { Rectangular: new BoardTypeRectangular(10, 10, [4, 5]) },
            { Placement: new Placement(
                0,
                AlterationTypeSimple.WholePlacement,
                PlacementAreaSimple.Half,
                [{ TotalLimit: { limit: 10 } }]
            ) },
            [VictoryConditionsSimple.AllCaptured, { GoalCount: { amount: 2, valid_pieces: [0] } }]
        )
    }
}
