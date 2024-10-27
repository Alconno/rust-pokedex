import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
    location: "/admin/users",
    get: {
        tags: ["admin_users"],
        summary: "Get paginated users",
        parameters: [
            {
                in: "query",
                name: "search",
                schema: {
                    type: "string"
                }
            },
            {
                in: "query",
                name: "sort_by",
                schema: {
                    type: "string",
                    enum: ["Email", "FirstName", "LastName", "Role", "EmailVerifiedAt", "CreatedAt", "UpdatedAt"]
                }
            },
            {
                in: "query",
                name: "sort",
                schema: {
                    type: "string",
                    enum: ["ASC", "DESC"]
                }
            },
            {
                in: "query",
                name: "page",
                schema: {
                    type: "integer",
                    minimum: 1,
                }
            },
            {
                in: "query",
                name: "per_page",
                schema: {
                    type: "integer",
                    minimum: 1,
                }
            },
            {
                in: "query",
                name: "hide_staff",
                schema: {
                    type: "boolean"
                }
            },
            {
                in: "query",
                name: "hide_deleted",
                schema: {
                    type: "boolean"
                }
            }
        ],
        responses: {
            "200": {
                description: "Paginated users retrieved successfully",
                content: {
                    "application/json": {
                        schema: {
                            type: "array",
                            items: {
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
                }
            },
            "400": BadRequestError,
            "401": UnauthorizedError,
            "500": InternalServerError,
        },
    },
} as PathItemObject;
