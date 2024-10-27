import { PathItemObject } from "openapi-ts-builder/dist/types";
import InternalServerError from "../../../../components/responses/InternalServerError";
import UnauthorizedError from "../../../../components/responses/UnauthorizedError";
import NotFoundError from "../../../../components/responses/NotFoundError";

export default {
    location: "/admin/pokedexes/{user_pokedex_id}",
    delete: {
        tags: ["admin_pokedexes"],
        summary: "Delete a specific user pokedex",
        parameters: [
            {
                name: "user_pokedex_id",
                in: "path",
                description: "The unique identifier of the user pokedex",
                required: true,
                schema: { type: "string", format: "uuid" }
            }
        ],
        responses: {
            "200": {
                description: "Successful operation. Returns a success message",
                content: {
                    "application/json": {
                        schema: {
                            type: "object",
                            properties: {
                                message: { type: "string" }
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
