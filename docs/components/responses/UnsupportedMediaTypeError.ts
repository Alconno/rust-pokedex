import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "UnsupportedMediaTypeError",
  description: "UnsupportedMediaType error", // 415
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
              {
                "in:{}": "Gives you the list of acceptable mime types",
              },
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
        message: "Unsupported media",
        code: "upload",
        cause: "in:image/jpeg,image/png",
        payload: null,
      },
    },
  },
} as ResponseObject;

