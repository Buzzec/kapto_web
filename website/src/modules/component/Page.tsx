import {LoginState} from "./LoginState";
import {Nav} from "react-bootstrap";
import React from "react";

export enum Page{
    Home,
    CreateGame,
    GamesList,
    RulesetExplorer,
    RulesetBuilder,
    Admin,
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
        }
    }

    public can_see_page(login: LoginState): boolean{
        switch (this.page){
            // Anonymous can access
            case Page.Home:
            case Page.RulesetExplorer:
                return true;
            // Need to be logged in
            case Page.CreateGame:
            case Page.GamesList:
            case Page.RulesetBuilder:
                return login > 0;
            // Need to be admin/under construction
            case Page.Admin:
                return login > 1;
        }
    }

    public to_nav_link(): JSX.Element{
        return <Nav.Link href={this.page_address} key={this.page_num}>{this.page_name}</Nav.Link>
    }
}
