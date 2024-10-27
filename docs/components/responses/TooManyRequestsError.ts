import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "TooManyRequestsError",
  description: "Too many requests", // 429
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
        message: "Too many requests, retry in 58s",
        code: "chill_out",
        cause: null,
        payload: null,
      },
    },
  },
} as ResponseObject;

