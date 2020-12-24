import {LoginState, TestingParam} from "./LoginState";
import {Nav} from "react-bootstrap";
import React from "react";
import {get_param} from "../QueryParams";

export enum Page{
    Home,
    CreateGame,
    GamesList,
    RulesetExplorer,
    RulesetBuilder,
    Admin,
    Account,
    Login,
    BouncingBalls,
}

export class PageMetaInfo{
    page: Page;
    page_num: number;
    page_address: string;
    page_name: string;

    constructor(page: Page) {
        this.page = page;
        this.page_num = page.valueOf();
        this.page_address = location.protocol + "//" + location.host + "/";
        this.page_name = "UNASSIGNED NAME";
        switch (page){
            case Page.Home:
                this.page_address += "";
                this.page_name = "Home";
                break;
            case Page.CreateGame:
                this.page_address += "create_game.html";
                this.page_name = "Create Game";
                break;
            case Page.GamesList:
                this.page_address += "games_list.html"
                this.page_name = "Games List";
                break;
            case Page.RulesetExplorer:
                this.page_address += "ruleset_explorer.html"
                this.page_name = "Ruleset Explorer";
                break;
            case Page.RulesetBuilder:
                this.page_address += "ruleset_builder.html";
                this.page_name = "Ruleset Builder";
                break;
            case Page.Admin:
                this.page_address += "admin.html";
                this.page_name = "Admin";
                break;
            case Page.Account:
                this.page_address += "account.html";
                this.page_name = "Account";
                break;
            case Page.Login:
                this.page_address += "login.html";
                this.page_name = "Login";
                break;
            case Page.BouncingBalls:
                this.page_address += "bouncing_balls.html";
                this.page_name = "Bouncing Balls";
                break;
        }

        const testing_param = get_param(TestingParam)
        if (testing_param != null) {
            this.page_address += "?testing=" + testing_param;
        }
    }

    public can_see_page(login: LoginState): boolean {
        if (login == LoginState.AllAccess) {
            return true;
        }
        switch (this.page) {
            // Anonymous can access
            case Page.Home:
            case Page.RulesetExplorer:
            case Page.BouncingBalls:
                return true;
            // Need to be logged in
            case Page.CreateGame:
            case Page.GamesList:
            case Page.RulesetBuilder:
            case Page.Account:
                return login > 0;
            // Need to be admin/under construction
            case Page.Admin:
                return login > 1;
            // Only if not logged in
            case Page.Login:
                return login == 0;
        }
    }

    public to_nav_link(): JSX.Element{
        return <Nav.Link href={this.page_address} key={this.page_num}>{this.page_name}</Nav.Link>
    }
}
