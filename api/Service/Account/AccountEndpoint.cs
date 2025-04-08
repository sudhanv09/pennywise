using api.Model;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace api.Service.Account;

public static class AccountEndpoint
{
    public static void UserAccountEndpoint(this IEndpointRouteBuilder app)
    {
        app.MapGet("/login",
            async ([FromBody]UserService service, string email, string password) =>
            {
                await service.Login(email, password);
                return Results.Created();
            });

        app.MapPost("/register",
            async ([FromBody]UserService service, string username, string email, string password, string currency) =>
            {
                var success = await service.Register(email, username, password, currency);
                if (success.IsFailed)
                {
                    return Results.BadRequest(success.Errors.ToString());
                }

                return Results.Ok();
            });

        app.MapPost("/logout", async (SignInManager<User> service) =>
        {
            await service.SignOutAsync();
            return Results.Ok();
        });
    }
}