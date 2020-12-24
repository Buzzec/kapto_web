import React, {Component, RefObject} from "react";
import {Button, Card, Col, Form} from "react-bootstrap";
import {PacketError} from "../api/generic";
import {login} from "../api/user";
import {Page, PageMetaInfo} from "../Page";

type LoginComponentState = {
    logging_in: boolean,
    username_ref: RefObject<HTMLInputElement>,
    password_ref: RefObject<HTMLInputElement>,
    error?: PacketError,
}
export class LoginComponent extends Component<{}, LoginComponentState> {
    constructor(props: {}) {
        super(props);
        this.state = {
            logging_in: false,
            username_ref: React.createRef(),
            password_ref: React.createRef(),
        };

        this.LoginErrorText = this.LoginErrorText.bind(this);
        this.on_submit_click = this.on_submit_click.bind(this);
    }

    render(): JSX.Element {
        return <React.Fragment>
            <Card><Card.Body>
                <Form>
                    <Form.Row>
                        <Col><h4>Login</h4></Col>
                    </Form.Row>

                    <Form.Group controlId="formLoginUsername">
                        <Form.Label>Username</Form.Label>
                        <Form.Control type="text" placeholder="Username" ref={this.state.username_ref}/>
                    </Form.Group>
                    <Form.Group controlId="formLoginPassword">
                        <Form.Label>Password</Form.Label>
                        <Form.Control type="password" placeholder="Password" ref={this.state.password_ref}/>
                    </Form.Group>
                    <Form.Row>
                        <Col xs="2">
                            <Button
                                variant="primary"
                                disabled={this.state.logging_in}
                                onClick={this.state.logging_in ? undefined : this.on_submit_click}
                            >Login</Button>
                        </Col>
                        <Col>
                            <this.LoginErrorText/>
                        </Col>
                    </Form.Row>
                </Form>
            </Card.Body></Card>
        </React.Fragment>;
    }

    private LoginErrorText(): JSX.Element {
        if (this.state.error) {
            return <p className="text-danger">Login Failed: {this.state.error.error_text}</p>;
        } else {
            return <React.Fragment/>;
        }
    }
    private on_submit_click(_event: React.MouseEvent<HTMLButtonElement, MouseEvent>): void {
        if (this.state.username_ref.current && this.state.password_ref.current) {
            this.setState((prev_state) => {
                return {
                    logging_in: true,
                    username_ref: prev_state.username_ref,
                    password_ref: prev_state.password_ref,
                }
            });

            login(this.state.username_ref.current.value, this.state.password_ref.current.value).then((packet_result) => {
                if (packet_result) {
                    this.setState((prev_state) => {
                        console.error({login_error: packet_result})
                        return {
                            logging_in: false,
                            username_ref: prev_state.username_ref,
                            password_ref: prev_state.password_ref,
                            error: packet_result,
                        }
                    })
                } else {
                    window.location.replace(new PageMetaInfo(Page.Home).page_address);
                }
            })
        } else {
            throw "Some ref isn't set";
        }
    }
}
