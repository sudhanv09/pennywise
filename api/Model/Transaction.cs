namespace api.Model;

public class Transaction
{
    public string Id { get; set; }
    public string Title { get; set; }
    public string? Description { get; set; }
    public int Amount { get; set; }
    public DateTime Created { get; set; } = DateTime.Now;
    public TransactionType TransactionType { get; set; }
    public TransactionCategory Category { get; set; }
    
    public User User { get; set; }
    public Account Account { get; set; }
}

public enum TransactionType
{
    Income,
    Expense,
    Transfer,
}

public enum TransactionCategory
{
    Food,
    Shopping,
    Bills,
    Groceries,
    Home,
    Health,
    Other,
}