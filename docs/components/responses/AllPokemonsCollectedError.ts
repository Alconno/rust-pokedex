import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "AllPokemonsCollectedError",
  description: "Error when all available pokemons have been collected",
  content: {
    "application/json": {
      schema: {
        type: "object",
        properties: {
          message: {
            type: "string",
            nullable: false,
          },
          code: {
            type: "string",
            nullable: false,
          },
          cause: {
            type: "string",
            nullable: true,
          },
          payload: {
            nullable: true,
            type: "object",
            properties: {
              "{some_key}": {
                type: "string",
                nullable: false,
              },
            },
          },
        },
      },
      example: {
        message: "All pokemons collected",
        code: "All pokemons collected",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;
