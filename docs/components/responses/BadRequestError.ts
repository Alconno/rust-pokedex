import { ResponseObject } from "openapi-ts-builder/dist/types";

export default {
  id: "BadRequestError",
  description: "Error when something is wrong with the request sent",
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
                no_email_field:
                  "Request was received without email field that was expected, validation will probably throw error before we get here",
              },
              {
                no_password_field: "same as the above",
              },
              {
                no_email_attribute: "same as the above",
              },
              {
                no_password_attribute: "same as the above",
              },
              {
                registration_invalid_email:
                  "email provided for registration is invalid",
              },
              {
                registration_invalid_password:
                  "Password provided for registration is invalid",
              },
              {
                registration_invalid_otp_token:
                  "OTP Token provided for registration is invalid",
              },
              {
                registration_email_already_in_use:
                  "Email that user tries to register with is already in use",
              },
              {
                token_action_executed:
                  "any kind of verification process (email, reset password, freeze account) will get its actionable token, you'll get this error when token was already executed",
              },
              {
                token_action_expired:
                  "for the above mentioned token, this will be error when user tries using it after it has expired",
              },
              {
                active_user_cannot_be_deleted_permanently:
                  "Error that will be displayed in rare cases when super-admin tries to delete forever active user",
              },
              {
                "input is out of range": "Input is out of range",
              },
              {
                "no possible date and time matching input":
                  "No possible date and time matching input",
              },
              {
                "input is not enough for unique date and time":
                  "Input is not enough for unique date and time",
              },
              {
                "input contains invalid characters":
                  "Input contains invalid characters",
              },
              {
                "premature end of input": "Premature end of input",
              },
              {
                "trailing input": "Trailing input",
              },
              {
                "bad or unsupported format string":
                  "Bad or unsupported format string",
              },
              {
                "invalid_number_of_answers:{}":
                  "Error that will be returned when on any question based request you send invalid number of answers",
              },
              {
                invalid_otp_token: "Invalid OTP Token",
              },
              {
                cannot_reassign_only_admin:
                  "Error that is returned when the request would leave a company without any administrator users",
              },
              {
                cannot_expire_only_owner:
                  "Error that is returned when the request would leave a company without any administrator users",
              },
              {
                content_disposition:
                  "Error that is returned when the Content-Disposition header is not set",
              },
              {
                filename:
                  "Error that is returned when the File name is not sent in upload request",
              },
              {
                file_type_not_allowed: "Invalid File type",
              },
              {
                maximum_file_size_exceeded: "Maximum File size exceeded",
              },
              {
                cannot_delete_default:
                  "When user tries to delete company_pipeline if it is the default one, he will get this error and will need to assign different one as default",
              },
              {
                "uploader:content_disposition:unknown":
                  "Something is wrong with the upload data or its type",
              },
              {
                "uploader:filename:unknown":
                  "Something is wrong with the upload data or its type",
              },  
              {
                one_time_password_expired:
                  "One time password that is sent to users with disabled 2fa after first step of login has expired",
              },
              {
                bare_metal_product_item_already_assigned_to_bare_metal_product:
                  "When user tries to add bare metal product item that already exists on bare metal product",
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
        message: "Bad request",
        code: "general",
        cause: "invalid_json_payload",
        payload: null,
      },
    },
  },
} as ResponseObject;
