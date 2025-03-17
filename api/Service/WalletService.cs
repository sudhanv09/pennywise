using api.Data;
using api.Model;
using api.Model.DTO;
using Microsoft.EntityFrameworkCore;

namespace api.Service;

public class WalletService(AppDbContext dbContext)
{
    public async Task<User> GetUserWallet(string userId)
    {
        return await dbContext.Users.FindAsync(userId);
    }

    public async Task AddTransaction(TxDto transaction)
    {
        var newTx = new Transaction()
        {
            Id = Ulid.NewUlid().ToString(),
            Title = transaction.Title,
            Description = transaction.Description,
            Amount = transaction.Amount,
            Created = transaction.CreatedAt,
            TransactionType = transaction.Type,
            Category = transaction.Category,
        };

        dbContext.Transactions.Add(newTx);
        await dbContext.SaveChangesAsync();
    }

    public async Task<List<Transaction>> GetMonthlyTransactions(DateTime startDate,
        DateTime endDate)
    {
        return await dbContext.Transactions
            .Where(x => x.Created >= startDate && x.Created <= endDate)
            .ToListAsync();
    }
}