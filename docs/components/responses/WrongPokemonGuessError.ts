import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "WrongPokemonGuessError",
  description: "Error when the guessed Pokemon is wrong",
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
        message: "Wrong Pokemon Guess",
        code: "Wrong guess",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;
