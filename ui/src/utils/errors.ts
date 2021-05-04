import { showSuccess, showWarn } from "./dialog";
import { logger } from "./logger";

export const TippedError = new Error("the error is tipped.");
export const LoggedError = new Error("the error is logged.");

interface WithTipConfig {
  successMsg?: string | false | JSX.Element;
  errorMsg?: string | JSX.Element;
  onSuccess?: () => void;
}

export async function withTipP(p: Promise<unknown>, config: WithTipConfig = {}): Promise<void> {
  const successMsg = config.successMsg === undefined ? "保存成功" : config.successMsg;
  const errorMsg = config.errorMsg || "保存失败";
  try {
    await p;
    if (config.onSuccess) {
      config.onSuccess();
    }
    if (successMsg !== false) {
      showSuccess(successMsg);
    }
  }
  catch (e) {
    if (e === TippedError) {
      return;
    }
    showWarn(errorMsg);
    if (e === LoggedError) {
      return;
    }
    logger.warn(e);
  }
}

export function withTip(p: Promise<void>, config: WithTipConfig = {}): void {
  withTipP(p, config).catch(() => 0);
}
