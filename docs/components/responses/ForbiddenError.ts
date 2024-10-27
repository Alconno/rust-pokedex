import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "ForbiddenError",
  description: "Error when trying to access forbidden resource",
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
            enum: [
            
            ],
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
        message: "Forbidden error",
        code: "forbidden",
        cause: "permission_denied_cannot_do_on_self",
        payload: null,
      },
    },
  },
} as ResponseObject;

