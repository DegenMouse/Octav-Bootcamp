import * as readline from 'readline';

export interface Book {
    title: string,
    author: string,
    genre: string,
    stock: number,
}

// Still kinda uncertain about how it works
export function question(query: string): Promise<string> {
    return new Promise((resolve) => {
        rl.question(query, resolve);
    });
}

export const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
