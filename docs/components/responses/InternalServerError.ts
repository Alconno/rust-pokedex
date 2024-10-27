import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "InternalServerError",
  description: "Error when something is wrong on the server side",
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
        message: "Something went wrong",
        code: "server_error",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;
