using Microsoft.EntityFrameworkCore;
using ComprasDoMes.Models;
using ComprasDoMes.Models.Internacionalizations;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddDbContext<ComprasDoMesContext>( opt => opt.UseInMemoryDatabase("ComprasDoMes") );
builder.Services.AddSingleton<Internacionalization>();

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddControllers();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();
app.UseAuthentication();
app.UseAuthorization();
app.MapControllers();
app.Run();
