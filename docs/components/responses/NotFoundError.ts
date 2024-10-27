import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "NotFoundError",
  description: "Error when resource wasn't found",
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
        message: "Not found error",
        code: "not_found",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;

