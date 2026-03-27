import path from "node:path";

import { Brocode } from "../src/brocode";
import type { BrocodeConfigObject } from "../src/brocodeOptions";

export const brocodeExecPath = path.join(process.cwd(), "..", "..", "brocode-rs", "target", "debug", "brocode");

type CreateTestClientOptions = {
  apiKey?: string;
  baseUrl?: string;
  config?: BrocodeConfigObject;
  env?: Record<string, string>;
  inheritEnv?: boolean;
};

export type TestClient = {
  cleanup: () => void;
  client: Brocode;
};

export function createMockClient(url: string): TestClient {
  return createTestClient({
    config: {
      model_provider: "mock",
      model_providers: {
        mock: {
          name: "Mock provider for test",
          base_url: url,
          wire_api: "responses",
          supports_websockets: false,
        },
      },
    },
  });
}

export function createTestClient(options: CreateTestClientOptions = {}): TestClient {
  const env =
    options.inheritEnv === false ? { ...options.env } : { ...getCurrentEnv(), ...options.env };

  return {
    cleanup: () => {},
    client: new Brocode({
      brocodePathOverride: brocodeExecPath,
      baseUrl: options.baseUrl,
      apiKey: options.apiKey,
      config: mergeTestProviderConfig(options.baseUrl, options.config),
      env,
    }),
  };
}

function mergeTestProviderConfig(
  baseUrl: string | undefined,
  config: BrocodeConfigObject | undefined,
): BrocodeConfigObject | undefined {
  if (!baseUrl || hasExplicitProviderConfig(config)) {
    return config;
  }

  // Built-in providers are merged before user config, so tests need a custom
  // provider entry to force SSE against the local mock server.
  return {
    ...config,
    model_provider: "mock",
    model_providers: {
      mock: {
        name: "Mock provider for test",
        base_url: baseUrl,
        wire_api: "responses",
        supports_websockets: false,
      },
    },
  };
}

function hasExplicitProviderConfig(config: BrocodeConfigObject | undefined): boolean {
  return config?.model_provider !== undefined || config?.model_providers !== undefined;
}

function getCurrentEnv(): Record<string, string> {
  const env: Record<string, string> = {};

  for (const [key, value] of Object.entries(process.env)) {
    if (key === "BROCODE_INTERNAL_ORIGINATOR_OVERRIDE") {
      continue;
    }
    if (value !== undefined) {
      env[key] = value;
    }
  }

  return env;
}
