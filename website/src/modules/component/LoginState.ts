import {get_user} from "../api/user";

export const TestingParam = "testing";

export enum LoginState {
    AllAccess = -1,
    LoggedOut = 0,
    LoggedInUser = 1,
    LoggedInAdmin = 2,
}

export function get_login_state(testing?: boolean): LoginState {
    if (testing !== undefined && testing) {
        return LoginState.AllAccess;
    }
    const user = get_user();
    if (user == null) {
        return LoginState.LoggedOut;
    }
    if (user.is_admin) {
        return LoginState.LoggedInAdmin;
    } else {
        return LoginState.LoggedInUser;
    }
}
