import React from "react";
import ReactDOM from "react-dom";
import {NavigationBar} from "./modules/component/NavigationBar";
import {Page} from "./modules/Page";
import {LoginComponent} from "./modules/component/LoginComponent";
import {Col, Container, Row} from "react-bootstrap";
import {RegistrationComponent} from "./modules/component/RegistrationComponent";

ReactDOM.render(
    <React.Fragment>
        <NavigationBar current_selection={Page.Login}/>
        <Container>
            <Row>
                <Col><LoginComponent/></Col>
                <Col><RegistrationComponent/></Col>
            </Row>
        </Container>
    </React.Fragment>,
    document.getElementById("root")
)
