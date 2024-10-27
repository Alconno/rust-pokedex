import { PathItemObject, ParameterObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
    location: "/admin/pokedexes",
    get: {
        tags: ["admin_pokedexes"],
        summary: "Get paginated user pokedexes",
        parameters: [
            {
                in: "query",
                name: "sort_by",
                schema: {
                    type: "string",
                    enum: ["created_at", "updated_at"]
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
                    minimum: 1
                }
            },
            {
                in: "query",
                name: "per_page",
                schema: {
                    type: "integer",
                    minimum: 1
                }
            }
        ],
        responses: {
            "200": {
                description: "Paginated user pokedexes retrieved successfully",
                content: {
                    "application/json": {
                        schema: {
                            type: "array",
                            items: {
                                type: "object",
                                properties: {
                                    id: { type: "string", format: "uuid" },
                                    user_id: { type: "string", format: "uuid" },
                                    pokemon_id: { type: "string", format: "uuid" },
                                    created_at: { type: "string", format: "date-time" },
                                    updated_at: { type: "string", format: "date-time" }
                                },
                                required: ["id", "user_id", "pokemon_id", "created_at", "updated_at"]
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
