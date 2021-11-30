export function generate_2fa_secret(): string;
export function check_2fa_code(secret: string, token: string): boolean;
