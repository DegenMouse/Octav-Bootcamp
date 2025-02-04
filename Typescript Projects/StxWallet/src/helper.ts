import * as readline from 'readline';

export function question(query: string): Promise<string> {
    return new Promise((resolve) => {
        rl.question(query, resolve);
    });
}

export const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});