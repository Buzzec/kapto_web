import React, {Component} from "react";

export class StyleSheet extends Component<{}, {}>{
    constructor(props: {}) {
        super(props);
    }

    render(): JSX.Element {
        return <React.Fragment>
            <link
                rel="stylesheet"
                href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css"
                integrity="sha384-9aIt2nRpC12Uk9gS9baDl411NQApFmC26EwAOH8WgZl5MYYxFfc+NcPb1dKGj7Sk"
                crossOrigin="anonymous"
            />
        </React.Fragment>;
    }
}
