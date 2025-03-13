using api.Data;
using api.Model;
using Microsoft.EntityFrameworkCore;

namespace api.Service;

public class WalletService
{
    private AppDbContext _context;
    public WalletService(AppDbContext dbContext)
    {
        _context = dbContext;
    }

    public async Task<User> GetUserWallet(string userId)
    {
        return await _context.Users.FindAsync(userId);
    }

    public async Task AddTransaction(Transaction transaction)
    {
        _context.Transactions.Add(transaction);
        await _context.SaveChangesAsync();
    }
    
}