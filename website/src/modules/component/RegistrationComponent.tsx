import React, {Component, RefObject} from "react";
import {Button, Card, Col, Form} from "react-bootstrap";
import {PacketError} from "../api/generic";
import {register} from "../api/user";
import {Page, PageMetaInfo} from "../Page";

type RegistrationComponentState = {
    registering: boolean,
    username_ref: RefObject<HTMLInputElement>,
    email_ref: RefObject<HTMLInputElement>,
    password_ref: RefObject<HTMLInputElement>,
    password_ref2: RefObject<HTMLInputElement>,
    error?: PacketError,
}
export class RegistrationComponent extends Component<{}, RegistrationComponentState> {
    constructor(props: {}) {
        super(props);
        this.state = {
            registering: false,
            username_ref: React.createRef(),
            email_ref: React.createRef(),
            password_ref: React.createRef(),
            password_ref2: React.createRef(),
        };

        this.RegisterErrorText = this.RegisterErrorText.bind(this);
        this.on_register_click = this.on_register_click.bind(this);
    }

    render(): JSX.Element {
        return <React.Fragment>
            <Card><Card.Body>
                <Form>
                    <Form.Row>
                        <Col><h4>Register</h4></Col>
                    </Form.Row>

                    <Form.Group controlId="formRegisterUsername">
                        <Form.Label>Username</Form.Label>
                        <Form.Control type="text" placeholder="Username" ref={this.state.username_ref}/>
                    </Form.Group>
                    <Form.Group controlId="formRegisterEmail">
                        <Form.Label>Email</Form.Label>
                        <Form.Control type="text" placeholder="Email" ref={this.state.email_ref}/>
                    </Form.Group>
                    <Form.Group controlId="formRegisterPassword">
                        <Form.Label>Password</Form.Label>
                        <Form.Control type="password" placeholder="Password" ref={this.state.password_ref}/>
                    </Form.Group>
                    <Form.Group controlId="formRegisterPassword2">
                        <Form.Control type="password" placeholder="Re-Type Password" ref={this.state.password_ref2}/>
                    </Form.Group>
                    <Form.Row>
                        <Col xs="2">
                            <Button
                                variant="primary"
                                disabled={this.state.registering}
                                onClick={this.state.registering ? undefined : this.on_register_click}
                            >Register</Button>
                        </Col>
                        <Col>
                            <this.RegisterErrorText/>
                        </Col>
                    </Form.Row>
                </Form>
            </Card.Body></Card>
        </React.Fragment>;
    }

    private RegisterErrorText(): JSX.Element {
        if (this.state.error) {
            return <p className="text-danger">Register Failed: {this.state.error.error_text}</p>;
        } else {
            return <React.Fragment/>;
        }
    }
    private on_register_click(_event: React.MouseEvent<HTMLButtonElement, MouseEvent>): void {
        if (this.state.username_ref.current
            && this.state.email_ref.current
            && this.state.password_ref.current
            && this.state.password_ref2.current) {
            this.setState((prev_state) => {
                return {
                    registering: true,
                    username_ref: prev_state.username_ref,
                    email_ref: prev_state.email_ref,
                    password_ref: prev_state.password_ref,
                    password_ref2: prev_state.password_ref2,
                }
            });
            const username = this.state.username_ref.current.value;
            const email = this.state.email_ref.current.value;
            const password = this.state.password_ref.current.value;
            const password2 = this.state.password_ref2.current.value;

            if (password != password2) {
                this.setState((prev_state) => {
                    return {
                        registering: false,
                        username_ref: prev_state.username_ref,
                        email_ref: prev_state.email_ref,
                        password_ref: prev_state.password_ref,
                        password_ref2: prev_state.password_ref2,
                        error: new PacketError("Passwords do not match"),
                    }
                });
                return;
            }

            register(username, email, password).then((packet_result) => {
                // @ts-ignore
                if ("error_text" in packet_result) {
                    this.setState((prev_state) => {
                        console.error({login_error: packet_result})
                        return {
                            registering: false,
                            username_ref: prev_state.username_ref,
                            email_ref: prev_state.email_ref,
                            password_ref: prev_state.password_ref,
                            password_ref2: prev_state.password_ref2,
                            error: packet_result,
                        };
                    })
                } else {
                    window.alert("Successfully Registered, please login");
                    window.location.replace(new PageMetaInfo(Page.Login).page_address);
                }
            })
        } else {
            throw "Some ref isn't set";
        }
    }
}
