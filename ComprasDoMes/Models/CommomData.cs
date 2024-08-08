namespace ComprasDoMes.Models.CommomDataModel;
using ComprasDoMes.Models.Internacionalizations;

public abstract class CommomData
{
    public string Id { get; init; } = Guid.NewGuid().ToString();
    public required string Name { get; set; }
    public DateTime Creation { get; } = DateTime.Now;
}

public abstract class CommomUserData : CommomData
{
    public InternacionalizationLanguage Language { get; set; } = InternacionalizationLanguage.Pt_BR;
}