export enum LoginState{
    LoggedOut = 0,
    LoggedInUser = 1,
    LoggedInAdmin = 2,
}

export function get_login_state(): LoginState{
    //TODO: Make this not static
    return LoginState.LoggedInAdmin;
}
