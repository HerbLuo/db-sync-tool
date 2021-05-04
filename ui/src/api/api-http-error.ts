export enum ErrorCodeEnum {
  AuthFailed = 578,
}

interface HttpErrorData {
  code: ErrorCodeEnum;
  serial: string;
  tip?: string;
}

export interface HttpError extends HttpErrorData {
  status: number;
}

export function makeHttpError(status: number, data: HttpErrorData): HttpError {
  return {
    status,
    ...data,
  };
}
