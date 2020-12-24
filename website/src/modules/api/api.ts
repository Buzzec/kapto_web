import {AuthToken} from "./AuthToken";
import {RequestData, ResponseData} from "./generic";
import {get_auth_token} from "./user";

export type ApiRequest = TokenPacket<RequestData>;
export type ApiResponse = TokenPacket<ResponseData>;

export function NewApiRequest(data: RequestData): ApiRequest {
    return new TokenPacket<RequestData>(get_auth_token(), data);
}
export function NewApiResponse(token: AuthToken | null, data: ResponseData): ApiResponse {
    return new TokenPacket<ResponseData>(token, data);
}

export class TokenPacket<T> {
    token: AuthToken | null;
    data: T;

    constructor(token: AuthToken | null, data: T) {
        this.token = token;
        this.data = data;
    }
}

const api_endpoint = location.protocol + "//" + location.host + "/api";

export async function make_request(request: ApiRequest): Promise<ApiResponse> {
    let response = await fetch(api_endpoint, {
        method: "POST",
        mode: "same-origin",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(request),
    });
    return await response.json() as ApiResponse;
}
