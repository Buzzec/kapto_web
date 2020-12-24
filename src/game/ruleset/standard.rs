use std::collections::HashMap;

use crate::game::Coordinate;
use crate::game::direction::Directions;
use crate::game::ruleset::{BoardType, Ruleset};
// use crate::game::ruleset::board_type::space::Space;
use crate::game::ruleset::color::ColorBuiltIn;
use crate::game::ruleset::piece_definition::{CaptureRequirement, CaptureRule, CaptureTarget, CaptureTimingRule, GoalMovementRule, JumpLimit, JumpRule, MoveRule, PieceDefinition};
use crate::game::ruleset::shape::Shape;
use crate::game::ruleset::starting_positions::StartingPositions;
use crate::game::ruleset::victory_condition::VictoryCondition;

// use ndarray::array;

pub fn standard_rules() -> Ruleset {
    Ruleset {
        name: "Standard".to_string(),
        seats: 2,
        allies: vec![vec![0, 1].into_iter().collect()],
        seat_colors: vec![ColorBuiltIn::Red.into(), ColorBuiltIn::Blue.into()],
        pieces: get_pieces(),
        board_type: get_board(),
        starting_positions: get_starting_positions(),
        victory_conditions: vec![
            VictoryCondition::AllCaptured,
            VictoryCondition::GoalCount {
                amount: 2,
                valid_pieces: vec![0, 1].into_iter().collect(),
            }
        ].into_iter().collect(),
    }
}

fn get_pieces() -> Vec<PieceDefinition> {
    let capture_rules: HashMap<_, _> = vec![(CaptureRule::JumpOver, CaptureTarget::EnemyOnly)].into_iter().collect();
    let big = PieceDefinition {
        name: "Big".to_string(),
        capture_rules: capture_rules.clone(),
        jump_rule: JumpRule::NoSameStart,
        capture_timing_rule: CaptureTimingRule::AfterTurn,
        capture_requirement: CaptureRequirement::Forced(10),
        jump_limit: JumpLimit::Unlimited { directions: Directions::ALL },
        move_rule: MoveRule::AnyDirection { limit: 1, directions: Directions::ALL },
        goal_move_rule: GoalMovementRule::Free,
        shape: Shape::Circle,
        size: 0.9,
        outline_color: ColorBuiltIn::Black.into(),
    };

    let small = PieceDefinition {
        name: "Little".to_string(),
        capture_rules,
        jump_rule: JumpRule::NoSameStart,
        capture_timing_rule: CaptureTimingRule::AfterTurn,
        capture_requirement: CaptureRequirement::None,
        jump_limit: JumpLimit::Limited { limit: 1, directions: Directions::ALL },
        move_rule: MoveRule::AnyDirection { limit: 1, directions: Directions::ALL },
        goal_move_rule: GoalMovementRule::Free,
        shape: Shape::Square,
        size: 0.6,
        outline_color: ColorBuiltIn::Black.into(),
    };

    vec![big, small]
}
fn get_board() -> BoardType {
    BoardType::Rectangular {
        rows: 10,
        columns: 10,
        goal_locations: vec![4, 5],
    }
    // BoardType::Custom(
    //     array![
    //     [Space::Normal, Space::Invalid, Space::Normal],
    //     [Space::Normal, Space::Normal, Space::Normal],
    //     [Space::Normal, Space::Normal, Space::Normal],
    //     [Space::Normal, Space::Normal, Space::Normal]
    //     ]
    // )
}
fn get_starting_positions() -> StartingPositions {
    // StartingPositions::MirroredFlipped(vec![
    //     vec![
    //         Coordinate::new(1, 1),
    //         Coordinate::new(2, 2),
    //         Coordinate::new(7, 2),
    //         Coordinate::new(8, 1),
    //     ],
    //     vec![
    //         Coordinate::new(2, 1),
    //         Coordinate::new(3, 2),
    //         Coordinate::new(3, 3),
    //         Coordinate::new(3, 4),
    //         Coordinate::new(4, 4),
    //         Coordinate::new(4, 5),
    //         Coordinate::new(3, 5),
    //         Coordinate::new(3, 6),
    //         Coordinate::new(3, 7),
    //         Coordinate::new(2, 8),
    //     ]
    // ])
    StartingPositions::NotMirrored(vec![
        vec![vec![
            Coordinate::new(1, 1),
            Coordinate::new(2, 2),
            Coordinate::new(7, 2),
            Coordinate::new(8, 1),
        ], vec![]],
        vec![vec![], vec![
            Coordinate::new(2, 1),
            Coordinate::new(3, 2),
            Coordinate::new(3, 3),
            Coordinate::new(3, 4),
            Coordinate::new(4, 4),
            Coordinate::new(4, 5),
            Coordinate::new(3, 5),
            Coordinate::new(3, 6),
            Coordinate::new(3, 7),
            Coordinate::new(2, 8),
        ]]
    ])
}

#[cfg(test)]
mod test{
    use crate::game::ruleset::standard::standard_rules;

    #[test]
    fn standard_rules_verify(){
        standard_rules().verify().expect("Ruleset is invalid!");
    }

    #[test]
    #[ignore]
    fn print_standard(){
        println!("{}", serde_json::to_string_pretty(&standard_rules()).expect("Could not serialize"));
    }
}
