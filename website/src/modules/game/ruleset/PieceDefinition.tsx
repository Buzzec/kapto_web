import {Directions} from "./Directions";
import {Shape, SimpleShape} from "./Shape";
import {Color} from "./Color";

export class PieceDefinition {
    name: string;
    capture_rules: CaptureRules;
    jump_rule: JumpRule;
    capture_timing_rule: CaptureTimingRule;
    capture_requirement: CaptureRequirement;
    jump_limit: JumpLimit;
    move_rule: MoveRule;
    goal_move_rule: GoalMovementRule;
    shape: Shape;
    size: number;
    color: Color;

    constructor(name: string, capture_rules: CaptureRules, jump_rule: JumpRule, capture_timing_rule: CaptureTimingRule, capture_requirement: CaptureRequirement, jump_limit: JumpLimit, move_rule: MoveRule, goal_move_rule: GoalMovementRule, shape: Shape, size: number, color: Color) {
        this.name = name;
        this.capture_rules = capture_rules;
        this.jump_rule = jump_rule;
        this.capture_timing_rule = capture_timing_rule;
        this.capture_requirement = capture_requirement;
        this.jump_limit = jump_limit;
        this.move_rule = move_rule;
        this.goal_move_rule = goal_move_rule;
        this.shape = shape;
        this.size = size;
        this.color = color;
    }
    static default(): PieceDefinition{
        return new PieceDefinition(
            "",
            {},
            JumpRule.Open,
            CaptureTimingRule.AfterTurn,
            CaptureRequirementSimple.None,
            JumpLimitSimple.Cannot,
            MoveRuleSimple.None,
            GoalMovementRule.Free,
            SimpleShape.Square,
            1.0,
            new Color(0, 0, 0, 1),
        )
    }
}

type CaptureRules = {
    JumpOver?: CaptureTarget,
    JumpOn?: CaptureTarget,
    Move?: CaptureTarget,
}
export enum CaptureTarget {
    EnemyOnly = "EnemyOnly",
    OwnOnly = "OwnOnly",
    All = "All",
}

export enum JumpRule {
    NoBacktracking = "NoBacktracking",
    NoSameStart = "NoSameStart",
    Open = "Open",
}

export enum CaptureTimingRule {
    AfterJump = "AfterJump",
    AfterTurn = "AfterTurn",
}

export type CaptureRequirement =
    CaptureRequirementSimple
    | { Forced: bigint }
export enum CaptureRequirementSimple {
    None = "None",
}

export type JumpLimit =
    JumpLimitSimple
    | { Unlimited?: { directions: Directions } }
    | { Limited?: { limit: bigint, directions: Directions } };
export enum JumpLimitSimple {
    Cannot = "Cannot",
}

export type MoveRule = MoveRuleSimple
    | { SameDirection?: MoveRuleInner }
    | { AnyDirection?: MoveRuleInner };
export enum MoveRuleSimple {
    None = "None",
}
export type MoveRuleInner = {
    limit: bigint;
    directions: Directions;
}

export enum GoalMovementRule {
    Locked = "Locked",
    OnlyToGoal = "OnlyToGoal",
    Free = "Free",
}

export function test() {
    console.log(JSON.stringify(PieceDefinition.default()));
}
