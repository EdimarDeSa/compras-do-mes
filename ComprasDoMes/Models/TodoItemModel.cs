namespace ComprasDoMes.Models;

public class TodoItem
{
    public long Id { get; init; }
    public string? Title { get; set; }
    public string? Description { get; set; }
    public bool IsComplete { get; set; } = false;
    public string? Secret { get; set; }
}

public class TodoItemDTO
{
    public long Id { get; init; }
    public string? Title { get; set; }
    public string? Description { get; set; }
    public bool IsComplete { get; set; } = false;
}