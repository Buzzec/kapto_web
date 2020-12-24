import React, {Component} from "react";
import {Page, PageMetaInfo} from "./Page";
import {Nav, Navbar} from "react-bootstrap";
import {StyleSheet} from "./StyleSheet";
import {get_login_state, TestingParam} from "./LoginState";
import {get_param} from "../QueryParams";

export type NavigationBarProps = { current_selection: Page }
export class NavigationBar extends Component<NavigationBarProps, {}> {
    constructor(props: NavigationBarProps) {
        super(props);

        this.NavLinks = this.NavLinks.bind(this);
    }

    render(): JSX.Element {
        return <React.Fragment>
            <StyleSheet/>
            <Navbar sticky="top">
                <Navbar.Brand href={"./index.html"}>Kapto Web</Navbar.Brand>
                <Navbar.Collapse>
                    <this.NavLinks/>
                </Navbar.Collapse>
            </Navbar>
        </React.Fragment>;
    }

    private NavLinks(): JSX.Element{
        const content = Object.values(Page)
            // Remove the names
            .filter(page_id => !isNaN(Number(page_id)))
            // Turn into metadata
            .map(page_id => new PageMetaInfo(page_id as Page))
            // Filter out unseeable pages
            .filter(meta_data => meta_data.can_see_page(get_login_state(get_param(TestingParam) != null)))
            // Turn data into a nav link
            .map(meta_data => meta_data.to_nav_link());
        return <Nav variant="tabs"
                    activeKey={new PageMetaInfo(this.props.current_selection).page_address}
                    className="mr-auto">
            {content}
        </Nav>
    }
}


