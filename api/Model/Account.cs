namespace api.Model;

public class Account
{
    public string Id { get; set; }
    public string Name { get; set; }
    public Transaction[] Transactions { get; set; }
    
    public User User { get; set; }
}