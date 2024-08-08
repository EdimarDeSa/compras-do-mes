using Microsoft.EntityFrameworkCore;
using ComprasDoMes.Models;
using ComprasDoMes.Models.Internacionalizations;
using System.Text.Json.Serialization;
using ComprasDoMes.Models.UserModel;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddDbContext<ComprasDoMesContext>( opt => opt.UseInMemoryDatabase("ComprasDoMes") );
builder.Services.AddSingleton<Internacionalization>();
builder.Services.AddSingleton<UserValidations>();

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

builder.Services.AddControllers()
    .AddJsonOptions( options => 
    options.JsonSerializerOptions.Converters.Add(new JsonStringEnumConverter())
    );

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
