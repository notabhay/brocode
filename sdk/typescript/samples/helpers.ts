import path from "node:path";

export function brocodePathOverride() {
  return (
    process.env.BROCODE_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "brocode-rs", "target", "debug", "brocode")
  );
}
