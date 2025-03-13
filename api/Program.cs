using api.Data;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateSlimBuilder(args);

builder.Services.AddDbContext<AppDbContext>(services => services.UseSqlite(builder.Configuration["ConnectionStrings:SqliteDefault"]));

var app = builder.Build();
app.Run();
