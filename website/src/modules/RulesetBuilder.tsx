import React, {Component} from "react";
import {Ruleset} from "./game/ruleset/Ruleset";
import {Color} from "./game/ruleset/Color";
import {PieceDefinition} from "./game/ruleset/PieceDefinition";

class State {
    ruleset: Ruleset;
    seats_text: string;
    pieces_count_text: string;

    constructor(ruleset: Ruleset) {
        this.ruleset = ruleset;
        this.seats_text = ruleset.seats.toString();
        this.pieces_count_text = ruleset.pieces.length.toString();
    }
}

export class RulesetBuilder extends Component<{}, State> {
    constructor(props: {}) {
        super(props);
        this.state = new State(Ruleset.default());

        this.handle_change_name = this.handle_change_name.bind(this);
        this.handle_change_seats = this.handle_change_seats.bind(this);
        this.handle_pieces_count_change = this.handle_pieces_count_change.bind(this);
    }
    render() {
        return (
            <React.Fragment>
                <h1>Ruleset Builder</h1>
                <table>
                    <thead>
                    <tr>
                        <th>Item</th>
                        <th>Input</th>
                        <th>Value</th>
                        <th>Error</th>
                    </tr>
                    </thead>
                    <tbody>
                    <InputRow item_name={"Name"}
                              input_value={this.state.ruleset.name}
                              real_value={this.state.ruleset.name}
                              error={<NoError/>}
                              change_function={this.handle_change_name}/>
                    <InputRow item_name={"Seats"}
                              input_value={this.state.seats_text}
                              change_function={this.handle_change_seats}
                              real_value={this.state.ruleset.seats.toString()}
                              error={<ErrorLabel display={RulesetBuilder.can_parse_positive_int(this.state.seats_text)}
                                                 label_text={"Could not parse integer"}/>}/>
                    <ValueRow item_name={"Allies"}
                              value={"[" + this.state.ruleset.allies.toString() + "]"}
                              error={<NoError/>}/>
                    <ValueRow item_name={"Seat Colors"}
                              value={<RulesetBuilder.SeatColors colors={this.state.ruleset.seat_colors}/>}
                              error={<NoError/>}/>
                    <InputRow item_name={"Pieces Count"}
                              input_value={this.state.pieces_count_text}
                              change_function={this.handle_pieces_count_change}
                              real_value={this.state.ruleset.pieces.length.toString()}
                              error={<ErrorLabel display={RulesetBuilder.can_parse_positive_int(this.state.pieces_count_text)}
                                                 label_text={"Could not parse integer"}/>}/>
                    </tbody>
                </table>
                <div>

                </div>
            </React.Fragment>
        );
    }

    handle_change_name(event: React.ChangeEvent<HTMLInputElement>) {
        this.setState((prev) => {
            const out: State = JSON.parse(JSON.stringify(prev));
            out.ruleset.name = event.target.value;
            return out;
        })
    }
    handle_change_seats(event: React.ChangeEvent<HTMLInputElement>) {
        this.setState((prev) => {
            const out: State = JSON.parse(JSON.stringify(prev));
            out.seats_text = event.target.value;
            const parse = parseInt(event.target.value);
            if (!isNaN(parse) && parse > 0) {
                out.ruleset.seats = parse;
            }
            return out;
        })
    }
    handle_pieces_count_change(event: React.ChangeEvent<HTMLInputElement>){
        this.setState((prev) => {
            const out: State = JSON.parse(JSON.stringify(prev));
            out.pieces_count_text = event.target.value;
            const parse = parseInt(event.target.value);
            if (!isNaN(parse) && parse > 0) {
                if (out.ruleset.pieces.length < parse){
                    while(out.ruleset.pieces.length < parse){
                        out.ruleset.pieces.push(PieceDefinition.default());
                    }
                }
                else{
                    out.ruleset.pieces.slice(0, parse);
                }
            }
            return out;
        })
    }

    private static SeatColors(props: { colors: Color[] }): JSX.Element{
        let id = 0;
        let contents = props.colors.map(color => {
            id += 1;
            return <span style={{ color: color.color}} key={id}>▉</span>;
        })
        return <React.Fragment>
            {contents}
        </React.Fragment>;
    }
    private static can_parse_positive_int(text: string): boolean{
        let parsed = parseInt(text);
        return isNaN(parsed) || parsed <= 0;
    }
}

function InputRow(props: {
    item_name: string,
    input_value: string,
    change_function: (event: React.ChangeEvent<HTMLInputElement>) => void,
    real_value: string,
    error: JSX.Element
}): JSX.Element {
    return <tr>
        <td>{props.item_name}</td>
        <td><input type="text" value={props.input_value} onChange={props.change_function}/></td>
        <td>{props.real_value}</td>
        <td>{props.error}</td>
    </tr>;
}

function ValueRow(props: {
    item_name: string,
    value: string | JSX.Element,
    error: JSX.Element
}): JSX.Element {
    return <tr>
        <td>{props.item_name}</td>
        <td/>
        <td>{props.value}</td>
        <td>{props.error}</td>
    </tr>
}

function ErrorLabel(props: { display: boolean, label_text: string }): JSX.Element {
    if (props.display) {
        return <label style={{color: "red"}}>{props.label_text}</label>;
    } else {
        return NoError();
    }
}
function NoError(): JSX.Element {
    return <label style={{color: "green"}}>No Error</label>;
}
