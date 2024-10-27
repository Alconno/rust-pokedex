import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import NotFoundError from "../../../../components/responses/NotFoundError";

export default {
    location: "/admin/pokemons/{pokemon_id}/abilities/{pokemon_ability_id}/update",
    put: {
        tags: ["admin_pokemons"],
        summary: "Update a specific pokemon ability",
        parameters: [
            {
                name: "pokemon_id",
                in: "path",
                description: "The unique identifier of the Pokemon",
                required: true,
                schema: { type: "string", format: "uuid" }
            },
            {
                name: "pokemon_ability_id",
                in: "path",
                description: "The unique identifier of the Pokemon ability",
                required: true,
                schema: { type: "string", format: "uuid" }
            }
        ],
        requestBody: {
            required: true,
            content: {
                "application/json": {
                    schema: {
                        type: "object",
                        properties: {
                            name: { type: "string" }
                        }
                    }
                }
            }
        },
        responses: {
            "200": {
                description: "Successful operation. Returns the updated pokemon ability details",
                content: {
                    "application/json": {
                        schema: {
                            type: "object",
                            properties: {
                                id: { type: "string", format: "uuid" },
                                pokemon_id: { type: "string", format: "uuid" },
                                name: { type: "string" },
                                created_at: { type: "string", format: "date-time" },
                                updated_at: { type: "string", format: "date-time" }
                            }
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
