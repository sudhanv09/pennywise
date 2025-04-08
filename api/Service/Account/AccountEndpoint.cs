namespace api.Service.Account;

public static class AccountEndpoint
{
    public static void UserAccountEndpoint(this IEndpointRouteBuilder app)
    {
        app.MapGet("/login", async (context) =>
        {
            await context.Response.WriteAsync("Hello World!");
        });
    }
}