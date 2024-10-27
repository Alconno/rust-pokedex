import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "PayloadTooLargeError",
  description: "Payload sent was too large", // 413
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
        message: "Payload too large",
        code: "upload",
        cause: "max_bytes:5242880",
        payload: null,
      },
    },
  },
} as ResponseObject;
