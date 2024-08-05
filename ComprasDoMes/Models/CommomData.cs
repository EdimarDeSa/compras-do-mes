namespace ComprasDoMes.Models.CommomDataModel;

public abstract class CommomData
{
    public required string Id { get; set; }
    public required string Name { get; set; }
    public DateTime Creation { get; } = DateTime.Now;
}

public abstract class CommomDataDTO
{
    public required string Id { get; init; }
    public required string Name { get; set; }
    // public DateTime Creation { get; set; }
} 