import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import NotFoundError from "../../../../components/responses/NotFoundError";

export default {
    location: "/admin/users/{user_id}/logout",
    patch: {
        tags: ["admin_users"],
        summary: "Logout a user",
        parameters: [
            {
                name: "user_id",
                in: "path",
                description: "User ID",
                required: true,
                schema: { type: "string" }
            }
        ],
        responses: {
            "200": {
                description: "User logged out successfully",
                content: {
                    "application/json": {
                        schema: {
                            type: "object",
                            properties: {
                                id: { type: "string", format: "uuid" },
                                email: { type: "string", format: "email" },
                                first_name: { type: "string" },
                                last_name: { type: "string" },
                                password: { type: "string" },
                                role: { type: "string" },
                                email_verified_at: { type: "string", format: "date-time" },
                                refresh_token: { type: ["string", "null"] },
                                created_at: { type: "string", format: "date-time" },
                                updated_at: { type: "string", format: "date-time" },
                                deleted_at: { type: ["string", "null"] }
                            },
                            required: ["id", "email", "first_name", "last_name", "password", "role", "email_verified_at", "created_at", "updated_at"]
                        }
                    }
                }
            },
            "401": UnauthorizedError,
            "404": NotFoundError,
            "500": InternalServerError,
        },
    },
} as PathItemObject;
