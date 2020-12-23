export type VictoryCondition =
    VictoryConditionsSimple
    | { GoalCount: { amount: number, valid_pieces: number[] } }
    | { PointDifference: number };
export enum VictoryConditionsSimple{
    AllCaptured = "AllCaptured",
}
