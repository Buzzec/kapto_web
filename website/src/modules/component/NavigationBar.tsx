import React, {Component} from "react";
import {Page, PageMetaInfo} from "../Page";
import {Button, Nav, Navbar} from "react-bootstrap";
import {StyleSheet} from "./StyleSheet";
import {get_login_state, TestingParam} from "../LoginState";
import {get_param} from "../QueryParams";
import {get_user, logout} from "../api/user";

export type NavigationBarProps = { current_selection: Page }
export class NavigationBar extends Component<NavigationBarProps, {}> {
    constructor(props: NavigationBarProps) {
        super(props);

        this.NavLinks = this.NavLinks.bind(this);
    }

    private static LoggedInText(): JSX.Element {
        const user = get_user();
        if (user) {
            return <React.Fragment>
                <Navbar.Collapse className="justify-content-end">
                    <Navbar.Text className="mr-1">
                        Signed in as: <a href={new PageMetaInfo(Page.Account).page_address}>{user.username}</a>
                    </Navbar.Text>
                    <Navbar.Text>
                        <Button variant="outline-secondary" size="sm" onClick={NavigationBar.logout}>Logout</Button>
                    </Navbar.Text>
                </Navbar.Collapse>
            </React.Fragment>;
        } else {
            return <React.Fragment/>;
        }
    }
    private static logout(_event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        logout();
        window.location.replace(new PageMetaInfo(Page.Home).page_address);
    }
    render(): JSX.Element {
        return <React.Fragment>
            <StyleSheet/>
            <Navbar sticky="top" bg="light" variant="light">
                <Navbar.Brand href={"./index.html"}>Kapto Web</Navbar.Brand>
                <Navbar.Collapse>
                    <this.NavLinks/>
                </Navbar.Collapse>
                <NavigationBar.LoggedInText/>
            </Navbar>
            <br/>
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
        return <Nav activeKey={new PageMetaInfo(this.props.current_selection).page_address}
                    className="mr-auto">
            {content}
        </Nav>
    }
}


