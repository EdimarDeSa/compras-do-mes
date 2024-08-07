namespace ComprasDoMes.Models.CommomDataModel;

public abstract class CommomData
{
    public string Id { get; init; } = Guid.NewGuid().ToString();
    public required string Name { get; set; }
    public DateTime Creation { get; } = DateTime.Now;
}