/**
 * TypeScript discriminated union for Docker status that matches Rust enum serialization.
 * 
 * Uses `#[serde(tag = "type")]` in Rust which serializes to JSON with a "type" field.
 * This enables compile-time type safety and runtime type checking.
 * 
 * References:
 * - [TypeScript Discriminated Unions](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#discriminated-unions)
 * - [Serde Enum Representations](https://serde.rs/enum-representations.html)
 */

export type DockerStatusPayload =
  | { type: "Running"; version: string }
  | { type: "Stopped" }
  | { type: "Error"; message: string }
  | { type: "Initializing" };

/**
 * Processed Docker status for UI consumption.
 * Extracts and normalizes data from the discriminated union.
 */
export interface ProcessedDockerStatus {
  status: "Running" | "Stopped" | "Error" | "Initializing" | "Unknown";
  color: string;
  version: string | null;
  message: string | null;
}

/**
 * Processes Docker status payload into UI-friendly format.
 * 
 * Maps technical states to user-friendly status:
 * - "Running" → "Running" (with version)
 * - "Stopped", "Error", "Initializing" → "Stopped" (user doesn't need technical details)
 * 
 * @param payload - Raw Docker status from Rust backend
 * @returns Processed status for UI rendering
 */
export function processDockerPayload(payload: DockerStatusPayload): ProcessedDockerStatus {
  switch (payload.type) {
    case "Running":
      return {
        status: "Running",
        color: "#22c55e", // Green
        version: payload.version,
        message: null,
      };
    case "Stopped":
    case "Error":
    case "Initializing":
      // All non-running states are treated as "Stopped" for user-friendly display
      return {
        status: "Stopped",
        color: "#ef4444", // Red
        version: null,
        message: null,
      };
    default:
      return {
        status: "Stopped",
        color: "#ef4444", // Red
        version: null,
        message: null,
      };
  }
} 