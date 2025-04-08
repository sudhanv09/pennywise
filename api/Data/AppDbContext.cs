using api.Model;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

namespace api.Data;

public class AppDbContext(DbContextOptions<AppDbContext> options) : IdentityDbContext<User>(options)
{
    public DbSet<User> Users { get; set; }
    public DbSet<Account> Accounts { get; set; }
    public DbSet<Transaction> Transactions { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        modelBuilder.Entity<IdentityUserLogin<string>>()
            .HasKey(login => new { login.LoginProvider, login.ProviderKey });
        
        modelBuilder.Entity<IdentityUserRole<string>>()
            .HasKey(role => new { role.UserId, role.RoleId });

        modelBuilder.Entity<IdentityUserToken<string>>()
            .HasKey(token => new { token.UserId, token.LoginProvider, token.Name });
        
        modelBuilder.Entity<User>()
            .Property(c => c.Currency)
            .HasDefaultValue("USD");
    }
}