import * as fs from 'fs';
import { Book, question, rl } from './helper';

var bookstore: Book[] = [];

function saveBooks() {
    const jsonData = JSON.stringify(bookstore,null,2);
    fs.writeFileSync('bookstore.json', jsonData, 'utf-8');

}

function loadBooks() {
    const jsonData = fs.readFileSync('bookstore.json', 'utf8');
    bookstore = JSON.parse(jsonData);
}

async function addBook() {
    const title = await question('Title: ');
    const author = await question('Author: ');
    const genre = await question('Genre: ');
    const stock = parseInt(await question('Stock: '), 10);

    const newBook: Book = {
        title,
        author,
        genre,
        stock,
    }

    bookstore.push(newBook);
    saveBooks();
    console.log("Book added successfully!");
}

function viewBooks() {
    if (bookstore.length === 0) {
        console.log("No books in bookstore.");
        return;
    }

    console.log("\nCurrent bookstore:");
    for (const book of bookstore)
    {
        console.log(`Title: ${book.title}`);
        console.log(`Author: ${book.author}`);
        console.log(`Genre: ${book.genre}`);
        console.log(`Stock: ${book.stock}\n`);
    }
}

async function deleteBook() {
    const titleToDelete = await question('Please enter the title: ')
    const index = bookstore.findIndex(book => book.title === titleToDelete);
    if (index !== -1) bookstore.splice(index, 1), console.log("Succesfully deleted this title");
    else
    console.log("This book title doesnt exist in the bookstore");
    saveBooks();

}

async function sortBooks() {
    const sortOption = await question("Sort book by: ");
    switch (sortOption) {
        case 'title':
            bookstore.sort((a, b) => a.title.localeCompare(b.title));
            break;
        case 'author':
            bookstore.sort((a, b) => a.author.localeCompare(b.author));
            break;
        case 'genre':
            bookstore.sort((a, b) => a.genre.localeCompare(b.genre));
            break;
        case 'stock':
            bookstore.sort((a, b) => b.stock - a.stock);
            break;
        default:
            console.log("Invalid sort option");
            break;
    }
    saveBooks();
}

async function modifyBooks() {
    const title = await question("Title of the book to modify: ");
    const book = bookstore.find(book => book.title = title)
    console.log(book);
    const bookIndex = bookstore.findIndex(book => book.title === title);
    if (!book) return;

    console.log("Please select the modify type: ")
    console.log("[1] Title");
    console.log("[2] Author");
    console.log("[3] Genre");
    console.log("[4] Stock");
    console.log("[0] Exit")
    const option = await question("Option: ");
    switch (option) {
        case '1':
            bookstore[bookIndex] = await modifyBookEdit("title",book);
            break;
        case '2':
            bookstore[bookIndex] = await modifyBookEdit("author",book);
            break;
        case '3':
            bookstore[bookIndex] = await modifyBookEdit("genre",book);
            break;
        case '4':
            {
                const value = parseInt(await question("Enter the new stock"),10);
                bookstore[bookIndex].stock = value;
            }
            break;
        case '0':
            console.log("Goodbye!");
            rl.close();
            return;
        default:
            console.log("Invalid option. Please try again.");
    }
    saveBooks();
}

async function modifyBookEdit<T extends keyof Book>(option: T, book: Book): Promise<Book> {
    const value = await question(`Enter the new ${option}: `);
    book[option] = value as Book[T];
    return book;
}

function printMenu() {
    console.log("\n=== bookstore Management ===");
    console.log("[1] Add a new book");
    console.log("[2] View books");
    console.log("[3] Sort books");
    console.log("[4] Delete book")
    console.log("[5] Modify books")
    console.log("[0] Exit");
}

async function menu() {
    while(true) {
        printMenu();
        
        const option = await question("Option: ");
        switch (option) {
            case '1':
                await addBook();
                break;
            case '2':
                viewBooks();
                break;
            case '3':
                await sortBooks();
                break;
            case '4':
                await deleteBook();
                break;
            case '5':
                await modifyBooks();
                break;
            case '0':
                console.log("Goodbye!");
                rl.close();
                return;
            default:
                console.log("Invalid option. Please try again.");
        }
    }
}

async function main() {
    loadBooks();
    await menu();
}

main();