export type User = {
    Id: string,
    Name: string,
    Email: string,
    Transactions: Transaction[],
    Accounts: Account[]
}

export type Transaction = {
    Id: string,
    Type: 'Expense' | 'Income',
    Name: string,
    Description: string | null,
    Amount: number,
    Created: Date,
    Category: string,
    Attachment: string | null,
} 

export type Account = {
    Name: string,
    Transactions: Transaction[],
    Currency: string,
}