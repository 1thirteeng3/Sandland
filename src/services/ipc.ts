import { invoke } from "@tauri-apps/api/core";
import type { SandlandError } from "../types/errors";

export function extractErrorMessage(error: unknown): string {
  if (typeof error === "string") return error;
  if (typeof error === "object" && error !== null) {
    const err = error as Record<string, unknown>;
    if (typeof err.message === "string" && err.message) return err.message;
    if (err.details) {
      if (typeof err.details === "string") return `${err.code ? String(err.code) + ": " : ""}${err.details}`;
      if (typeof err.details === "object" && err.details !== null) {
        const details = err.details as Record<string, unknown>;
        if (typeof details.reason === "string") return `${String(err.code || "Erro")}: ${details.reason}`;
        return JSON.stringify(err.details);
      }
    }
    if (err.code) return `Erro [${String(err.code)}]`;
  }
  return String(error);
}

export async function invokeSafe<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    const msg = extractErrorMessage(error);
    if (typeof error === "object" && error !== null && "code" in error) {
      const sandErr = error as SandlandError;
      sandErr.message = sandErr.message || msg;
      throw sandErr;
    }
    throw {
      code: "IoError",
      message: msg,
      details: error,
    } as SandlandError;
  }
}

export const invokeCommand = invokeSafe;

