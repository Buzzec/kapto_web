import React from "react";
import ReactDOM from "react-dom";
import {NavigationBar} from "./modules/component/NavigationBar";
import {Page} from "./modules/Page";

ReactDOM.render(
    <React.Fragment>
        <NavigationBar current_selection={Page.RulesetBuilder}/>
    </React.Fragment>,
    document.getElementById("root")
)
