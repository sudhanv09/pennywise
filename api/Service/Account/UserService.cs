using api.Model;
using FluentResults;
using Microsoft.AspNetCore.Identity;

namespace api.Service.Account;

public class UserService
{
    private readonly UserManager<User> _userManager;
    private readonly SignInManager<User> _signInManager;

    public UserService(UserManager<User> userManager, SignInManager<User> signInManager)
    {
        _userManager = userManager;
        _signInManager = signInManager;
    }

    public async Task<Result> Register(string email, string username, string password, string preferredCurrency)
    {
        var user = new User()
        {
            UserName = username,
            Email = email,
            Currency = preferredCurrency
        };
        var result = await _userManager.CreateAsync(user, password);
        if (!result.Succeeded)
        {
            return Result.Fail(result.Errors.ToString());
        }
        
        await _signInManager.SignInAsync(user, isPersistent: false);
        return Result.Ok();
    }

    public async Task<Result> Login(string email, string password)
    {
        var user = await _userManager.FindByEmailAsync(email);
        if (user == null)
        {
            return Result.Fail("Invalid username or password.");
        }
        
        var result = await _signInManager.PasswordSignInAsync(user, password, false, false);
        if (!result.Succeeded)
            return Result.Fail("Invalid username or password.");
        
        return Result.Ok();
    }
}