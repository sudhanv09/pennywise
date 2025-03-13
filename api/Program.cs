using System.Text.Json.Serialization;
using api.Data;
using api.Model;
using api.Service;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateSlimBuilder(args);

builder.Services.AddScoped<WalletService>();
builder.Services.AddDbContext<AppDbContext>(services =>
    services.UseSqlite(builder.Configuration["ConnectionStrings:SqliteDefault"])
);

builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonContext.Default);
});

var app = builder.Build();

// Controllers
app.MapGet("/", () => "Hello World!");

app.MapGet("/wallet", async (WalletService service, string id) => await service.GetUserWallet(id));
app.MapPost("/wallet/new", async (WalletService service, Transaction tx) =>
{
    await service.AddTransaction(tx);
    return Results.Created();
});

app.Run();

[JsonSerializable(typeof(User))]
[JsonSerializable(typeof(Transaction))]
[JsonSerializable(typeof(WalletService))]
internal partial class AppJsonContext : JsonSerializerContext
{
}