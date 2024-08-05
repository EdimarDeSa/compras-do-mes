using Microsoft.EntityFrameworkCore;
using ComprasDoMes.Models.UserModel;

namespace ComprasDoMes.Models;
public interface IComprasDoMesContext
{
    public DbSet<User> Users { get; set; }
}

public class ComprasDoMesContext : DbContext, IComprasDoMesContext
{
    public ComprasDoMesContext(DbContextOptions<ComprasDoMesContext> options) : base(options)
    {}

    public DbSet<User> Users { get; set; }
}
