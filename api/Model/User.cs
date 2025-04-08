using Microsoft.AspNetCore.Identity;

namespace api.Model;

public class User: IdentityUser
{
    public Account[]? Accounts { get; set; }
    public Transaction[]? Transactions { get; set; }
    public string Currency { get; set; } = "USD";

}