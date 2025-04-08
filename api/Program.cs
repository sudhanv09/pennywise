using System.Text.Json;
using System.Text.Json.Serialization;
using api.Data;
using api.Model;
using api.Model.DTO;
using api.Service;
using api.Service.Account;
using api.Service.Wallet;
using api.Utils;
using Microsoft.AspNetCore.Http.Json;
using Microsoft.AspNetCore.Identity;
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

builder.Services.AddAuthentication(IdentityConstants.ApplicationScheme).AddIdentityCookies();
builder.Services.AddAuthorizationBuilder();
builder.Services.AddDbContext<AppDbContext>(services =>
    services.UseSqlite(builder.Configuration["ConnectionStrings:SqliteDefault"])
);
builder.Services
    .AddIdentityCore<User>()
    .AddEntityFrameworkStores<AppDbContext>()
    .AddApiEndpoints();

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c => { c.SchemaFilter<EnumFilter>(); });

var app = builder.Build();

// Endpoints
app.UserAccountEndpoint();
app.UserWalletEndpoint();

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