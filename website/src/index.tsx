import React from "react";
import ReactDOM from "react-dom";
import {Accordion, Button, Card} from "react-bootstrap";
import {RulesetBuilder} from "./modules/RulesetBuilder";
import {NavigationBar} from "./modules/component/NavigationBar";
import {Page} from "./modules/component/Page";

ReactDOM.render(
    <React.Fragment>
        <NavigationBar current_selection={Page.Home}/>
        <Accordion defaultActiveKey="0">
            <Card>
                <Card.Header>
                    <Accordion.Toggle as={Button} variant="link" eventKey="0">
                        Click me!
                    </Accordion.Toggle>
                </Card.Header>
                <Accordion.Collapse eventKey="0">
                    <Card.Body>
                        <h1>Hello</h1>
                    </Card.Body>
                </Accordion.Collapse>
            </Card>
            <Card>
                <Card.Header>
                    <Accordion.Toggle as={Button} variant="link" eventKey="1">
                        Click me!
                    </Accordion.Toggle>
                </Card.Header>
                <Accordion.Collapse eventKey="1">
                    <Card.Body>Hello! I'm another body</Card.Body>
                </Accordion.Collapse>
            </Card>
        </Accordion>
        <RulesetBuilder/>
    </React.Fragment>
    ,
    document.getElementById("root")
)
