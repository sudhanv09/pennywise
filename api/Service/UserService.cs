using api.Data;
using api.Model;
using Microsoft.EntityFrameworkCore;

namespace api.Service;

public class UserService
{
    private readonly AppDbContext _context;

    public UserService(AppDbContext context)
    {
        _context = context;
    }

    public async Task<List<User>> GetAllUsers()
    {
        return await _context.Users.ToListAsync();
    }

    public async Task AddUser(string name, string? email, string? currency)
    {
        var newUser = new User()
        {
            Id = Ulid.NewUlid(),
            Name = name,
            Email = email,
            Currency = currency,
        };
        
        await _context.Users.AddAsync(newUser);
        await _context.SaveChangesAsync();
    }

    public async Task<User> GetUserById(string id)
    {
        var getUlid = Ulid.Parse(id);
        return await _context.Users.FindAsync(getUlid);
    }
}