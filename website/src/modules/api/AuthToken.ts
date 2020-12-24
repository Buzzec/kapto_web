export class AuthToken {
    id: number;
    token: Array<number>;

    constructor(id: number, token: Array<number>) {
        this.id = id;
        this.token = token;
    }
}
