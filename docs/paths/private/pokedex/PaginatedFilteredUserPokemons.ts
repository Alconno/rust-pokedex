import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../components/responses/UnauthorizedError";

export default {
    location: "/api/pokedex/{version}/pokedex/{user_id}",
    get: {
        tags: ["pokedex"],
        summary: "Get paginated user pokemons from the Pokedex",
        parameters: [
            {
                name: "version",
                in: "path",
                description: "API version",
                required: true,
                schema: { type: "string" }
            },
            {
                name: "user_id",
                in: "path",
                description: "User ID",
                required: true,
                schema: { type: "string" }
            }
        ],
        requestBody: {
            required: true,
            content: {
                "application/json": {
                    schema: {
                        type: "object",
                        properties: {
                            page: { type: "integer", minimum: 1, default: 1 },
                            page_size: { type: "integer", minimum: 1, default: 10 },
                            sort_by_field: { type: "string" },
                            sort_order: { type: "string", enum: ["ASC", "DESC"] },
                            filter_by_date: {
                                type: "object",
                                properties: {
                                    start_date: { type: "string", format: "date" },
                                    end_date: { type: "string", format: "date" }
                                },
                                required: ["start_date", "end_date"]
                            }
                        },
                        required: ["page", "page_size"] 
                    }
                }
            }
        },
        responses: {
            "200": {
                description: "Paginated user pokemons retrieved successfully",
                content: {
                    "application/json": {
                        schema: {
                            type: "object",
                            properties: {
                                pokemons: {
                                    type: "array",
                                    items: {
                                        type: "object",
                                        properties: {
                                            id: { type: "string", format: "uuid" },
                                            name: { type: "string" },
                                            base_experience: { type: "integer" },
                                            height: { type: "integer" },
                                            weight: { type: "integer" },
                                            is_default: { type: "boolean" },
                                            order: { type: "integer" },
                                            image: { type: "string", format: "url" },
                                            created_at: { type: "string", format: "date-time" },
                                            updated_at: { type: "string", format: "date-time" },
                                        },
                                        required: ["id", "name", "base_experience", "height", "weight", "is_default", "order", "image", "created_at", "updated_at"]
                                    }
                                }
                            },
                            required: ["pokemons"]
                        }
                    }
                }
            },
            "401": UnauthorizedError,
            "500": InternalServerError,
        },
    },
} as PathItemObject;
