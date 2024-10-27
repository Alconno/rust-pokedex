import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import BadRequestError from "../../../../components/responses/BadRequestError";

export default {
    location: "/admin/pokemons",
    get: {
        tags: ["admin_pokemons"],
        summary: "Get paginated pokemons",
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
                    enum: ["Name", "BaseExperience", "Height", "Weight", "CreatedAt", "UpdatedAt"]
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
            }
        ],
        responses: {
            "200": {
                description: "Paginated pokemons retrieved successfully",
                content: {
                    "application/json": {
                        schema: {
                            type: "array",
                            items: {
                                type: "object",
                                properties: {
                                    id: { type: "string", format: "uuid" },
                                    name: { type: "string" },
                                    base_experience: { type: "integer" },
                                    height: { type: "integer" },
                                    weight: { type: "integer" },
                                    pokemon_id: { type: "integer" },
                                    is_default: { type: "boolean" },
                                    order: { type: "integer" },
                                    image: { type: "string", format: "url" },
                                    created_at: { type: "string", format: "date-time" },
                                    updated_at: { type: "string", format: "date-time" }
                                },
                                required: ["id", "name", "base_experience", "height", "weight", "pokemon_id", "is_default", "order", "image", "created_at", "updated_at"]
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
