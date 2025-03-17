using System.Text.Json;
using System.Text.Json.Serialization;
using api.Data;
using api.Model;
using api.Service;
using api.Utils;
using Microsoft.AspNetCore.Http.Json;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Options;
using Scalar.AspNetCore;
using Swashbuckle.AspNetCore.SwaggerGen;

var builder = WebApplication.CreateSlimBuilder(args);


builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, AppJsonContext.Default);
});

// Fixes Swagger Enum generation
builder.Services.AddTransient<ISerializerDataContractResolver>(sp =>
{
    var opts = sp.GetRequiredService<IOptions<JsonOptions>>().Value?.SerializerOptions
               ?? new JsonSerializerOptions(JsonSerializerDefaults.Web);

    return new JsonSerializerDataContractResolver(opts);
});

builder.Services.AddScoped<UserService>();
builder.Services.AddScoped<WalletService>();
builder.Services.AddDbContext<AppDbContext>(services =>
    services.UseSqlite(builder.Configuration["ConnectionStrings:SqliteDefault"])
);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c => { c.SchemaFilter<EnumFilter>(); });

var app = builder.Build();

// Controllers
app.MapGet("/", () => "Hello World!");

app.MapPost("/new", async (UserService service, string username, string? email, string? currency) => await service.AddUser(username, email, currency));
app.MapGet("/users", async (UserService service) => await service.GetAllUsers());
app.MapGet("/user/{id}", async (UserService service, string id) => await service.GetUserById(id));

app.MapGet("/user/wallet", async (WalletService service, string id) => await service.GetUserWallet(id));
app.MapPost("/user/new", async (WalletService service, Transaction tx) =>
{
    await service.AddTransaction(tx);
    return Results.Created();
});

if (app.Environment.IsDevelopment())
{
    app.UseSwagger(options => { options.RouteTemplate = "openapi/{documentName}.json"; });
    app.MapScalarApiReference();
}

app.Run();

[JsonSerializable(typeof(User))]
[JsonSerializable(typeof(List<User>))]
[JsonSerializable(typeof(Transaction))]
[JsonSerializable(typeof(TransactionType))]
[JsonSerializable(typeof(TransactionCategory))]
[JsonSerializable(typeof(Account))]
internal partial class AppJsonContext : JsonSerializerContext
{
}