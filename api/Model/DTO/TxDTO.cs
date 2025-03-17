namespace api.Model.DTO;

public record TxDto(
    string? Title,
    string? Description,
    int Amount,
    TransactionType Type,
    TransactionCategory? Category,
    DateTime CreatedAt,
    Account? Account);