using api.Data;
using api.Model;
using FluentResults;
using Microsoft.EntityFrameworkCore;

namespace api.Service;

public class UserService(AppDbContext context)
{
    public async Task<List<User>> GetAllUsers()
    {
        return await context.Users.ToListAsync();
    }

    public async Task<Result> AddUser(string name, string? email, string? currency)
    {
        return Result.Ok();
    }
}