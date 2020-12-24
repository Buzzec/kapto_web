import {make_user_request, PacketError, PacketResult} from "./generic";
import {AuthToken} from "./AuthToken";
import {delete_cookie, get_cookie, set_cookie} from "../cookie";

export type UserRequest =
    { Register: { username: string, email: string, password: string } }
    | { Login: { username: string, password: string } }
    | { DeleteUser: { username: string, password: string } }
    | { GetUserInfo: UserIdentification };
export type UserResponse =
    { PartialUser: PartialUser }
    | { FullUser: FullUser };

export type UserIdentification =
    { Id: number }
    | { Username: string }
    | { Email: string };
export class PartialUser {
    id: number;
    username: string;
    is_admin: boolean;

    constructor(id: number, username: string, is_admin: boolean) {
        this.id = id;
        this.username = username;
        this.is_admin = is_admin;
    }
}
export class FullUser {
    id: number;
    username: string;
    email: string;
    is_admin: boolean;

    constructor(id: number, username: string, email: string, is_admin: boolean) {
        this.id = id;
        this.username = username;
        this.email = email;
        this.is_admin = is_admin;
    }
}

const user_cname = "user";
const auth_token_cname = "auth_token";
export function get_user(): null | FullUser {
    const cookie = get_cookie(user_cname);
    if (cookie == null) {
        return null;
    }
    return JSON.parse(cookie);
}
export function get_auth_token(): null | AuthToken {
    const cookie = get_cookie(auth_token_cname);
    if (cookie == null) {
        return null;
    }
    return JSON.parse(cookie);
}
export function logout(): void {
    delete_cookie(user_cname);
}

export async function register(username: string, email: string, password: string): Promise<PacketResult<FullUser>> {
    const response = await make_user_request({Register: {username: username, email: email, password: password}});
    if ("error_text" in response) {
        return response;
    }
    // @ts-ignore
    if ("User" in response.data) {
        if ("FullUser" in response.data.User) {
            return response.data.User.FullUser as FullUser;
        }
    }
    console.error({response: response});
    return new PacketError("Bad Packet");
}
export async function login(username: string, password: string): Promise<PacketResult<void>> {
    const response = await make_user_request({Login: {username: username, password: password}});
    console.log(response);
    if ("error_text" in response) {
        return response;
    }
    // @ts-ignore
    if ("User" in response.data) {
        if ("FullUser" in response.data.User) {
            if (response.token == null) {
                return new PacketError("Null Token");
            }
            set_cookie(user_cname, JSON.stringify(response.data.User.FullUser), 7);
            set_cookie(auth_token_cname, JSON.stringify(response.token), 7);
            return;
        }
    }
    console.error({response: response});
    return new PacketError("Bad Packet");
}
export async function delete_user(username: string, password: string): Promise<PacketResult<void>> {
    const response = await make_user_request({DeleteUser: {username: username, password: password}});
    if ("error_text" in response) {
        return response;
    }
    // @ts-ignore
    if (!("GenericSuccess" in response.data)) {
        console.error({response: response});
        return new PacketError("Bad Packet");
    }
}
export async function get_user_info(identification: UserIdentification): Promise<PacketResult<PartialUser | FullUser>> {
    const response = await make_user_request({GetUserInfo: identification});
    if ("error_text" in response) {
        return response;
    }
    // @ts-ignore
    if ("User" in response.data) {
        if ("PartialUser" in response.data.User) {
            return response.data.User.PartialUser
        }
        if ("FullUser" in response.data.User) {
            return response.data.User.FullUser
        }
    }
    console.error({response: response});
    return new PacketError("Bad Packet");
}
