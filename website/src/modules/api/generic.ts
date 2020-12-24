import {UserRequest, UserResponse} from "./user";
import {ApiResponse, make_request, NewApiRequest} from "./api";

export type RequestData =
    { Ping: number }
    | { User: UserRequest };
export type ResponseData =
    ResponseSimple
    | { Pong: number }
    | { GenericError: PacketError }
    | { User: UserResponse };
enum ResponseSimple {
    GenericSuccess = "GenericSuccess",
}

export function check_error(response: ResponseData): PacketResult<void> {
    // @ts-ignore
    if ("GenericError" in response) {
        return response.GenericError;
    }
}
export type PacketResult<T> = PacketError | T;
export class PacketError {
    data: string;
    error_text: string;

    constructor(error_text: string) {
        this.data = JSON.stringify({client_error: new Error(error_text)});
        this.error_text = error_text;
    }
}

export async function ping(value: number): Promise<PacketResult<number>> {
    const response = await make_request(NewApiRequest({Ping: value}));
    const error = check_error(response.data);
    if (error) {
        return error;
    }
    // @ts-ignore
    if ("Pong" in response.data) {
        return response.data.Pong;
    } else {
        console.error(response);
        return new PacketError("Bad Packet");
    }
}
export async function make_user_request(request: UserRequest): Promise<PacketResult<ApiResponse>> {
    const response = await make_request(NewApiRequest({User: request}));
    const error = check_error(response.data);
    if (error) {
        return error;
    }
    return response;
}
