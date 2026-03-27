export type BrocodeConfigValue = string | number | boolean | BrocodeConfigValue[] | BrocodeConfigObject;

export type BrocodeConfigObject = { [key: string]: BrocodeConfigValue };

export type BrocodeOptions = {
  brocodePathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the Brocode CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: BrocodeConfigObject;
  /**
   * Environment variables passed to the Brocode CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
