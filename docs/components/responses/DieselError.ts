import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "DieselError",
  description: "Error related to database operations (Diesel)",
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
        message: "DB Error",
        code: "db",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;
