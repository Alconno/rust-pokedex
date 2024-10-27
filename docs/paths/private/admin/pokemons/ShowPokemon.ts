import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import NotFoundError from "../../../../components/responses/NotFoundError";

export default {
    location: "/admin/pokemons/{pokemon_id}",
    get: {
        tags: ["admin_pokemons"],
        summary: "Get detailed information about a specific Pokemon",
        parameters: [
            {
                name: "pokemon_id",
                in: "path",
                description: "The unique identifier of the Pokemon",
                required: true,
                schema: { type: "string", format: "uuid" }
            }
        ],
        responses: {
            "200": {
                description: "Detailed information about the specified Pokemon",
                content: {
                    "application/json": {
                        schema: {
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
            },
            "401": UnauthorizedError,
            "404": NotFoundError,
            "500": InternalServerError,
        },
    },
} as PathItemObject;
