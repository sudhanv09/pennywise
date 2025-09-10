import * as v from "valibot";

export const registerSchema = v.pipe(
    v.object({
        name: v.string(),
        email: v.pipe(v.string(), v.email("Please enter a valid email address")),
        password: v.pipe(
            v.string(),
            v.minLength(8, "Password must be at least 8 characters long")
        ),
        confirmPassword: v.string(),
    }),
    v.forward(
        v.partialCheck(
            [["password"], ["confirmPassword"]],
            (input) => input.password === input.confirmPassword,
            "The two passwords don't match"
        ),
        ["confirmPassword"]
    )
);