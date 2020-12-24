const urlParams = new URLSearchParams(window.location.search);

export function get_param(param: string): string | null {
    return urlParams.get(param);
}
