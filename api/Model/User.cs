namespace api.Model;

public class User
{
    public Ulid Id { get; set; }
    public string Name { get; set; }
    public string? Email { get; set; }
    public Account[]? Accounts { get; set; }
    public Transaction[]? Transactions { get; set; }
    public string Currency { get; set; } = "USD";

}